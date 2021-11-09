use std::error::Error;
use std::path::PathBuf;

use structopt::StructOpt;

mod dirtools;

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
    let opt = Opt::from_args();
    let debug = opt.debug;
    let recurse = opt.recurse;
    let max_depth: usize = if recurse { usize::MAX } else { opt.max_depth };
    let path = &opt.path;

    // TODO remove / improve
    if debug {
        println!("{:?}", &opt);
    }

    let files = dirtools::find_files_recurse(&path, max_depth);

    println!("Paths:");
    files
        .into_iter()
        .flatten()
        .for_each(|e| println!("{}", e.display()));

    Ok(())
}
