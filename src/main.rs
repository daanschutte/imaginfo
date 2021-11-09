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
    /// Input file
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Enable debugging
    #[structopt(short, long)]
    debug: bool,

    /// Follow directories recursively
    #[structopt(short, long)]
    recurse: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let debug = opt.debug;

    // TODO remove / improve
    if debug {
        println!("{:?}", &opt);
    }

    let path = opt.path;
    let recurse = opt.recurse;

    let files = if recurse {
        dirtools::find_files_recurse(&path)
    } else {
        dirtools::find_files_no_recurse(&path)
    };

    println!("Dirs:");
    files.into_iter().for_each(|e| println!("{}", e.display()));

    Ok(())
}
