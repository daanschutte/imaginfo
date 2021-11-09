use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use structopt::StructOpt;
use walkdir::WalkDir;

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
        find_files_recurse(&path)
    } else {
        find_files_no_recurse(&path)
    };

    println!("Dirs:");
    files.into_iter().for_each(|e| println!("{}", e.display()));

    Ok(())
}

fn find_files_recurse(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden_walkdir(e))
        .filter(|e| !e.path_is_symlink())
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<PathBuf>>()
}

fn find_files_no_recurse(path: &Path) -> Vec<PathBuf> {
    fs::read_dir(&path)
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .filter(|e| !e.file_type().unwrap().is_dir())
        .filter(|e| !e.file_type().unwrap().is_symlink())
        .map(|e| e.path())
        .collect::<Vec<PathBuf>>()
}

// TODO make generic
fn is_hidden_walkdir(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
