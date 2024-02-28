use std::path::Path;

use roped::Strand;

use crate::{paste::functions::EntryType, state::State, truncate_path_string};

#[derive(Debug)]
pub struct CreateDirectory;

impl Strand for CreateDirectory {
    type State = State;

    fn run(state: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let path = input
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let new_dir = crate::offset_dir(&std::path::PathBuf::from(path))?;

        let print = truncate_path_string(&new_dir);

        match create(&new_dir, EntryType::File) {
            Ok(result) => {
                match result {
                    CreateContainer::File(file) => {
                        state.file = Some(file);
                        println!("Created file \"{}\", it is now opened", print);
                    },
                    CreateContainer::Folder => {
                        println!("Created directory \"{}\"", print);
                    },
                }
                Ok(())
            },
            Err(err) => {
                match err {
                    CreateError::CreateFile => Err(format!("Could not create file \"{}\"", print)),
                    CreateError::CreateFolder => Err(format!("Could not create directory \"{}\"", print)),
                    CreateError::AlreadyExists => Err(format!("Directory \"{}\" already exists", print)),
                    CreateError::InvalidType => Err("Invalid entry type".to_string()),
                }
            },
        }
    }
}

pub enum CreateContainer {
    File(std::fs::File),
    Folder,
}

pub enum CreateError {
    CreateFile,
    CreateFolder,
    AlreadyExists,
    InvalidType,
}

pub fn create(path: &Path, entry_type: EntryType) -> Result<CreateContainer, CreateError> {
    if path.exists() {
        return Err(CreateError::AlreadyExists);
    }

    match entry_type {
        EntryType::File => {
            let file = std::fs::File::create(path)
                .ok()
                .ok_or(CreateError::CreateFile)?;

            Ok(CreateContainer::File(file))
        }
        EntryType::Folder => {
            std::fs::create_dir_all(path)
                .ok()
                .ok_or(CreateError::CreateFolder)?;

            Ok(CreateContainer::Folder)
        },
        _ => Err(CreateError::InvalidType),
    }
}
