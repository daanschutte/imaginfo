use std::error::Error;
use std::path::PathBuf;

use exif::Exif;
use log::debug;
use structopt::StructOpt;

mod dir_tools;

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
        .map(|path| get_exif_data(&path))
        .filter_map(|e| e.ok())
        .collect::<Vec<Exif>>();

    Ok(())
}

fn get_exif_data(path: &PathBuf) -> Result<Exif, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    for f in exif.fields() {
        println!(
            "{} {} {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }

    Ok(exif)
}
