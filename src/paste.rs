mod functions;
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
    let meta = origin.metadata().map_err(|_| {
        format!(
            "Corrupted or missing copy directory \"{}\"",
            truncate_path_string(origin)
        )
    })?;
    let file_type = meta.file_type();

    let origin_type = if file_type.is_dir() {
        PathType::Folder
    } else if file_type.is_file() {
        PathType::File
    } else {
        return Err(format!(
            "Invalid copy directory \"{}\"",
            truncate_path_string(origin)
        ));
    };

    match (origin_type, move_type, move_info) {
        (PathType::Folder, MoveType::Copy, MoveInfo::Ident) => copy_folder_to(origin, destination),
        (PathType::Folder, MoveType::Copy, MoveInfo::Contents) => {
            copy_folder_contents_to(origin, destination)
        }
        (PathType::Folder, MoveType::Cut, MoveInfo::Ident) => move_item_to(origin, destination),
        (PathType::Folder, MoveType::Cut, MoveInfo::Contents) => {
            move_folder_contents_to(origin, destination)
        }
        (PathType::File, MoveType::Copy, MoveInfo::Ident) => copy_file_to(origin, destination),
        (PathType::File, MoveType::Copy, MoveInfo::Contents) => {
            copy_file_contents_to(origin, destination)
        }
        (PathType::File, MoveType::Cut, MoveInfo::Ident) => move_item_to(origin, destination),
        (PathType::File, MoveType::Cut, MoveInfo::Contents) => {
            move_file_contents_to(origin, destination)
        }
    }
}

enum PathType {
    Folder,
    File,
}
