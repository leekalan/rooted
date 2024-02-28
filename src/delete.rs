use std::path::Path;

use roped::Strand;

use crate::{paste::functions::EntryType, state::State, truncate_path_string};

#[derive(Debug)]
pub struct DeleteDirectory;

impl Strand for DeleteDirectory {
    type State = State;

    fn run(_: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let path = input
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let new_dir = crate::offset_dir(&std::path::PathBuf::from(path))?;
        delete(&new_dir);
        println!("Deleted \"{}\"", truncate_path_string(&new_dir));

        Ok(())
    }
}

pub fn delete(path: &Path) -> Option<()> {
    match EntryType::get(path) {
        EntryType::Folder => std::fs::remove_dir_all(path).ok(),
        _ => std::fs::remove_file(path).ok(),
    }
}
