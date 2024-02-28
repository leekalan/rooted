pub mod functions;
use functions::*;

use crate::*;

use std::path::Path;

use self::state::MoveType;

#[derive(Debug)]
pub struct PasteDirectory;

impl Strand for PasteDirectory {
    type State = State;

    fn run(state: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let (input_p, move_info) = if let Some(c) = input.chars().next() {
            if c == '#' {
                (unsafe { input.get_unchecked(1..) }, MoveInfo::Contents)
            } else {
                (input, MoveInfo::Ident)
            }
        } else {
            (input, MoveInfo::Ident)
        };

        let path = input_p
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let destination_dir = get_dir()?.join(std::path::PathBuf::from(path));

        let (copy_dir, move_type) = match &state.moving {
            state::Moving::None => return Err("No copy directory".to_string()),
            state::Moving::Move(p, m) => (p, m),
        };

        println!(
            "{}",
            paste_dir(copy_dir, &destination_dir, move_type, &move_info)?
        );

        Ok(())
    }
}

enum MoveInfo {
    Ident,
    Contents,
}

fn paste_dir(
    origin: &Path,
    destination: &Path,
    move_type: &MoveType,
    move_info: &MoveInfo,
) -> Result<String, String> {
    match (move_type, move_info) {
        (MoveType::Copy, MoveInfo::Ident) => {
            todo!()
        }
        (MoveType::Copy, MoveInfo::Contents) => {
            todo!()
        }
        (MoveType::Cut, MoveInfo::Ident) => {
            let name = origin.file_name().ok_or("Unnamed copy directory")?;

            let new_des = destination.join(name);

            move_entry_ref(origin, &new_des)?;

            Ok(format!(
                "Moved \"{}\" into \"{}\"",
                truncate_path_string(origin),
                truncate_path_string(destination)
            ))
        }
        (MoveType::Cut, MoveInfo::Contents) => {
            let origin_type = EntryType::get(origin);

            match origin_type {
                EntryType::Folder => move_folder_contents(origin, destination)?,
                _ => move_entry_ref(origin, destination)?,
            };

            Ok(format!(
                "Moved contents of \"{}\" into \"{}\"",
                truncate_path_string(origin),
                truncate_path_string(destination)
            ))
        }
    }
}
