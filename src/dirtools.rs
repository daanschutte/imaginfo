use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub(crate) fn find_files_recurse(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden_walkdir(e))
        .filter(|e| !e.path_is_symlink())
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<PathBuf>>()
}

pub(crate) fn find_files_no_recurse(path: &Path) -> Vec<PathBuf> {
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