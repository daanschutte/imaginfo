use std::error::Error;
use std::path::{Path, PathBuf};

use exif::Exif;
use log::{debug, error};
use structopt::StructOpt;

mod database;
mod dir;
mod exif_tools;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Imaginfo",
    about = "An application to give insights into photo metadata"
)]
struct Opt {
    /// Root directory from which to search
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Enable debug mode
    #[structopt(short = "D", long)]
    debug: bool,

    /// Print all RAW data from processed image files
    #[structopt(short = "i", long)]
    debug_image_info: bool,

    /// Follow links
    #[structopt(short, long)]
    follow_links: bool,

    /// Follow directories recursively up to the maximum depth, ignoring <max-depth>.
    #[structopt(short, long)]
    recurse: bool,

    /// Set maximum depth to search
    #[structopt(short = "d", long = "depth", default_value = "1")]
    max_depth: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let conn = database::get_connection("./db.db3")?;

    let opt = Opt::from_args();
    let debug = opt.debug;
    let debug_image_info = opt.debug_image_info;
    let follow_links = opt.follow_links;
    let recurse = opt.recurse;
    let path: &PathBuf = &opt.path;

    let max_depth: Option<usize> = if recurse {
        Option::None
    } else {
        Option::Some(opt.max_depth)
    };

    if debug {
        debug!("{:?}", &opt);
    }

    dir::find_files(path, debug, follow_links, max_depth)
        .unwrap()
        .iter()
        .map(|path| exif_tools::get_exif_data(path, debug_image_info))
        .map(log_error)
        .filter_map(|res| res.ok())
        .map(|(p, e)| exif_tools::exif_to_image(p, &e))
        .filter_map(|image| image.ok())
        .for_each(|i| database::insert_sony(&conn, &i));

    Ok(())
}

fn log_error(e: Result<(&Path, Exif), Box<dyn Error>>) -> Result<(&Path, Exif), ()> {
    e.map_err(|err| error!("{:?}", err))
}
