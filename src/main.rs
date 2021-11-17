use std::error::Error;
use std::path::PathBuf;

use exif::{Exif, Tag};
use log::{debug, error, info};
use structopt::StructOpt;

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

    /// Follow links
    #[structopt(short, long)]
    follow_links: bool,

    /// Include hidden files
    #[structopt(short = "H", long)]
    hidden: bool,

    /// Follow directories recursively up to the maximum depth
    #[structopt(short, long)]
    recurse: bool,

    /// Set maximum depth to search
    #[structopt(short = "d", long = "depth", default_value = "1")]
    max_depth: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let opt = Opt::from_args();
    let debug = opt.debug;
    let hidden = opt.hidden;
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

    let files = dir_tools::find_files(path, debug, follow_links, hidden, max_depth);

    let exif_data = files
        .unwrap()
        .iter()
        .map(|path| exfiltrate::get_exif_data(path, debug))
        .map(log_error)
        .filter_map(|exif| exif.ok())
        .collect::<Vec<Exif>>();

    let apertures: Vec<f64> = exif_data
        .iter()
        .map(|exif| exfiltrate::get_tag_rational(Tag::FNumber, exif))
        .flatten()
        .map(|e| e.to_f64())
        .collect::<Vec<f64>>();

    apertures.iter().for_each(|f| debug!("Aperture: f{:?}", f));

    Ok(())
}

fn log_error(e: Result<Exif, Box<dyn Error>>) -> Result<Exif, ()> {
    e.map_err(|err| error!("{:?}", err))
}
