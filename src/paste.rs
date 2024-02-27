use crate::*;

use std::{fs::{self, File}, path::Path};

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
            state::Moving::None => return Err("no directory copied".into()),
            state::Moving::Move(p, m) => (p, m),
        };

        println!("{}", paste_dir(&copy_dir, &destination_dir, move_type, move_info)?);

        Ok(())
    }
}
#[derive(PartialEq, Eq)]
enum PathType {
    Folder,
    File,
}

enum MoveInfo {
    Ident,
    Contents,
}

fn paste_dir(origin: &Path, destination: &Path, move_type: &MoveType, move_info: MoveInfo) -> Result<String, String> {
    let origin_type = if let Ok(meta) = origin.metadata() {
        if meta.is_file() {
            PathType::File
        } else if meta.is_dir() {
            PathType::Folder
        } else {
            return Err("Origin is not copyable".to_string())
        }
    } else {
        return Err("Origin does not exist".to_string())
    };

    if destination.exists() {
        let destination_type = if let Ok(meta) = destination.metadata() {
            if meta.is_file() {
                PathType::File
            } else if meta.is_dir() {
                PathType::Folder
            } else {
                return Err("Destination cannot be pasted too".to_string())
            }
        } else {
            return Err("Destination is corrupted".to_string())
        };

        if origin_type != destination_type { return Err("Origin and Destination do not match".to_string()) }

        println!("Destination already exists. Are you sure you want to add contents? (y/_)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).map_err(|_| "Cancelling..".to_string())?;
        if input.trim().to_lowercase() != "y" {
            return Ok("Cancelling..".to_string())
        }

        if origin_type == PathType::File {
            return copy_file_contents(origin, destination, move_type)
        }
    } else {
        match origin_type {
            PathType::Folder => fs::create_dir_all(origin).map_err(|_| "Could not create the specified destination".to_string())?,
            PathType::File => {
                return copy_file_contents(origin, destination, move_type)
            },
        }
    }

    match move_info {
        MoveInfo::Ident => todo!(),
        MoveInfo::Contents => todo!(),
    }
}

fn copy_file_contents(origin: &Path, destination: &Path, move_type: &MoveType) -> Result<String, String> {
    //Check if exist

    //let file = std::fs::File::create(origin).map_err(|_| "Could not create the specified destination".to_string())?;
    
    todo!()
}

fn paste_dir_contents(origin: &Path, destination: &Path, move_type: &MoveType) -> Result<String, String> {
    todo!()
}