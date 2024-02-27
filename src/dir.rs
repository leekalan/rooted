use std::path::{Path, PathBuf};

pub fn is_valid(path: &Path) -> bool {
    if let Ok(metadata) = std::fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}

pub fn get_dir() -> Result<PathBuf, String> {
    std::env::current_dir().map_err(|_| "Could not retrieve current directory".into())
}

pub fn set_dir(path: &Path) -> Result<(), String> {
    std::env::set_current_dir(path).map_err(|_| "Could not set current directory".into())
}

pub fn offset_dir(rel_path: &Path) -> Result<PathBuf, String> {
    let current_dir = get_dir()?;
    let new_dir_r = current_dir.join(rel_path);
    let new_dir = new_dir_r.canonicalize().map_err(|_| {
        format!(
            "Could not find directory \"{}\"",
            crate::truncate_path_string(&new_dir_r),
        )
    })?;
    Ok(new_dir)
}
