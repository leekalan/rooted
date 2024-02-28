use std::path::Path;

use roped::Strand;

use crate::{paste::functions::EntryType, state::State, truncate_path_string};

#[derive(Debug)]
pub struct DeleteDirectory;

impl Strand for DeleteDirectory {
    type State = State;

    fn run(_: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let (input_p, contents) = if let Some(c) = input.chars().next() {
            if c == '#' {
                (unsafe { input.get_unchecked(1..) }, true)
            } else {
                (input, false)
            }
        } else {
            (input, false)
        };

        let path = input_p
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let new_dir = crate::offset_dir(&std::path::PathBuf::from(path))?;

        let print = truncate_path_string(&new_dir);

        match contents {
            false => {
                delete(&new_dir).ok_or(format!("Could not delete \"{}\"", print))?;
                println!("Deleted \"{}\"", print);
                Ok(())
            }
            true => {
                delete_contents(&new_dir)
                    .ok_or(format!("Could not delete contents of \"{}\"", print))?;
                println!("Deleted contents of \"{}\"", print);
                Ok(())
            }
        }
    }
}

pub fn delete(path: &Path) -> Option<()> {
    match EntryType::get(path) {
        EntryType::Folder => std::fs::remove_dir_all(path).ok(),
        _ => std::fs::remove_file(path).ok(),
    }
}

pub fn delete_contents(path: &Path) -> Option<()> {
    for entry in path.read_dir().ok()? {
        if let Ok(entry) = entry {
            delete(&entry.path());
        }
    }

    Some(())
}
