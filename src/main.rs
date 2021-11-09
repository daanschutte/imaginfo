use std::error::Error;
use std::fs;
use std::fs::FileType;
use std::path::PathBuf;

use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, StructOpt)]
#[structopt(name = "Imaginfo", about = "An about section about Imaginfo")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    println!("{:?}", &opt);
    let path = opt.input;
    // let entries = find_files(&path);

    let recurse = find_files_recursive(&path);
    println!("Recursive dirs:");
    recurse
        .into_iter()
        .for_each(|e| println!("{}", e.display()));

    let no_recurse = find_files_no_recurse(&path);
    println!("Non recursive dirs:");
    no_recurse
        .into_iter()
        .for_each(|e| println!("{}", e.display()));

    Ok(())
}

fn find_files_recursive(path: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden_walkdir(e))
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<PathBuf>>()
}

fn find_files_no_recurse(path: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(&path)
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .filter(|e| !e.file_type().unwrap().is_dir())
        .map(|e| e.path())
        .collect::<Vec<PathBuf>>()
}

// TODO make generic
fn is_hidden_walkdir(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_hidden(entry: &std::fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
