use std::error::Error;
use std::fs;
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

    find_files_recursive(&path);
    find_files_no_recurse(&path);

    Ok(())
}

fn find_files_recursive(path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let entries = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<PathBuf>>();

    println!("Recursive dirs:");
    &entries.clone().into_iter().for_each(|e| println!("{}", e.display()));


    Ok(entries)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}


fn find_files_no_recurse(path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let entries = fs::read_dir(&path)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<PathBuf>>();


    println!("Non-recursive dirs:");
    &entries.clone().into_iter().for_each(|e| println!("{}", e.display()));

    Ok(entries)
}
