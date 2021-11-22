use std::error::Error;
use std::path::PathBuf;

use crate::database::get_connection;
use exif::{Exif, Tag};
use log::{debug, error, info};
use structopt::StructOpt;

mod database;
mod dir_tools;
mod exfiltrate;

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
    let _db = database::get_connection("./db.db3");

    let opt = Opt::from_args();
    let debug = opt.debug;
    let debug_image_info = opt.debug_image_info;
    let follow_links = opt.follow_links;
    let recurse = opt.recurse;
    let path = &opt.path;

    let max_depth: Option<usize> = if recurse {
        Option::None
    } else {
        Option::Some(opt.max_depth)
    };

    if debug {
        debug!("{:?}", &opt);
    }

    let files = dir_tools::find_files(path, debug, follow_links, max_depth);

    let exif_data = files
        .unwrap()
        .iter()
        .map(|path| exfiltrate::get_exif_data(path, debug_image_info))
        .map(log_error)
        .filter_map(|exif| exif.ok())
        .collect::<Vec<Exif>>();

    // TODO drop testing code
    let apertures = exif_data
        .iter()
        .map(|exif| exfiltrate::get_tag_rational(Tag::FNumber, exif))
        .flatten()
        .map(|e| e.to_f64())
        .collect::<Vec<f64>>();
    info!(
        "Average aperture: f{}",
        apertures.iter().sum::<f64>() / apertures.len() as f64
    );

    Ok(())
}

fn log_error(e: Result<Exif, Box<dyn Error>>) -> Result<Exif, ()> {
    e.map_err(|err| error!("{:?}", err))
}
