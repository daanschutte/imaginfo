use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub(crate) fn find_files_recurse(
    path: &Path,
    max_depth: usize,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
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

    Ok(paths)
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
