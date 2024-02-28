use std::path::Path;

use crate::paste::functions::EntryType;

pub fn delete(path: &Path) -> Option<()> {
    match EntryType::get(path) {
        EntryType::File => std::fs::remove_file(path).ok(),
        EntryType::Folder => std::fs::remove_dir_all(path).ok(),
        _ => None,
    }
}
