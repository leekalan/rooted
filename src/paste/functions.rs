use std::path::Path;

use crate::truncate_path_string;

pub fn copy_file_to(origin: &Path, destination: &Path) -> Result<String, String> {
    todo!()
}

pub fn copy_file_contents_to(origin: &Path, destination: &Path) -> Result<String, String> {
    todo!()
}

pub fn copy_folder_to(origin: &Path, destination: &Path) -> Result<String, String> {
    todo!()
}

pub fn copy_folder_contents_to(origin: &Path, destination: &Path) -> Result<String, String> {
    todo!()
}

pub fn move_file_to() {}

pub fn move_file_contents_to(origin: &Path, destination: &Path) -> Result<String, String> {
    todo!()
}

pub fn move_folder_to(origin: &Path, destination: &Path) -> Result<String, String> {
    let name = origin
        .file_name()
        .ok_or("Unnamed copy directory")?
        .to_string_lossy()
        .to_string();

    let new_des = destination.join(name);

    todo!()
}

pub fn move_folder_contents_to(origin: &Path, destination: &Path) -> Result<String, String> {
    if destination.exists() {
        todo!()

        //TODO
        //Delete original directory
    } else {
        todo!()
    }
}

enum EntryType {
    File,
    Folder,
    Other,
    None,
}
impl EntryType {
    pub fn get(path: &Path) -> Self {
        if path.exists() {
            if path.is_file() {
                EntryType::File
            } else if path.is_dir() {
                EntryType::Folder
            } else {
                EntryType::Other
            }
        } else {
            EntryType::None
        }
    }
}

fn move_entry_ref(origin: &Path, destination: &Path) -> Result<(), String> {
    let origin_type = EntryType::get(origin);
    let destination_type = EntryType::get(destination);

    match (origin_type, destination_type) {
        // Skips because the entry no longer exists
        (EntryType::None, _) => return Ok(()),
        // The destination does not exist so can be cleanly moved
        (_, EntryType::None) => match std::fs::rename(origin, destination) {
            Ok(_) => (),
            Err(_) => {
                println!(
                    "Unable to move \"{}\" to \"{}\", do you want to cancel? (y)",
                    truncate_path_string(origin),
                    truncate_path_string(destination)
                );
    
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read user input");
        
                let trimmed_input = input.trim().to_lowercase();
        
                if trimmed_input != "y" && trimmed_input != "yes" {
                    println!("Skipping...");
                    return Err("cancelled".to_string());
                }
            },
        },
        // Copy the contents as it is a folder
        (EntryType::Folder, EntryType::Folder) => todo!(),
        // Already exists so will be replaced
        (_, _) => todo!(),
    }

    if destination.exists() {
        println!(
            "Directory \"{}\" already exists, do you want to add the contents of \"{}\"? (y)",
            truncate_path_string(destination),
            truncate_path_string(origin)
        );

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let trimmed_input = input.trim().to_lowercase();

        if trimmed_input != "y" && trimmed_input != "yes" {
            println!("Skipping...");
            return Ok(());
        }

        move_entry_contents(origin, destination)?;
    } else {
        std::fs::rename(origin, destination).map_err(|_| "Invalid destination")?;
    }

    Ok(())
}

fn move_entry_contents(origin: &Path, destination: &Path) -> Result<(), String> {
    todo!()
}
