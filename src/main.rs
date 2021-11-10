use std::error::Error;
use std::path::PathBuf;

use exif::{Exif, Tag};
use log::debug;
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
    let recurse = opt.recurse;
    let max_depth: usize = if recurse { usize::MAX } else { opt.max_depth };
    let path = &opt.path;

    if debug {
        debug!("{:?}", &opt);
    }

    let files = dir_tools::find_files_recurse(path, debug, max_depth);

    let exif_data = &files
        .unwrap()
        .into_iter()
        .map(|path| exfiltrate::get_exif_data(&path))
        .filter_map(|e| e.ok())
        .collect::<Vec<Exif>>();

    let apertures = exif_data
        .into_iter()
        .map(|e| exfiltrate::get_tag_rational(Tag::FNumber, e))
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .map(|e| e.to_f64())
        .collect::<Vec<f64>>();

    apertures
        .clone()
        .into_iter()
        .for_each(|f| debug!("f: {:?}", f));

    Ok(())
}
