use std::path::{Path, PathBuf};

use log::{debug, info};
use walkdir::WalkDir;

pub(crate) fn find_files_recurse(
    path: &Path,
    debug: bool,
    max_depth: usize,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    info!(
        "Searching from {} with a max_depth of {}",
        &path.display(),
        max_depth
    );

    let paths = WalkDir::new(path)
        .min_depth(1)
        .max_depth(max_depth)
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|e| !is_hidden(e))
        .filter(|e| !e.path_is_symlink())
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<PathBuf>>();

    if debug {
        log_debug(&paths);
    }

    Ok(paths)
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn log_debug(paths: &[PathBuf]) {
    paths
        .to_owned()
        .into_iter()
        .for_each(|p| debug!("Found {}", p.display()));
}
