use std::path::Path;

use crate::truncate_path_string;

#[derive(PartialEq, Eq)]
pub enum EntryType {
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

pub fn move_entry_ref(origin: &Path, destination: &Path) -> Result<(), String> {
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
                    "| Unable to move \"{}\" to \"{}\", do you want to cancel? (y)",
                    truncate_path_string(origin),
                    truncate_path_string(destination)
                );

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read user input");

                let trimmed_input = input.trim().to_lowercase();

                if trimmed_input == "y" && trimmed_input == "yes" {
                    println!("| Cancelling...");
                    return Err("Cancelled".to_string());
                }
            }
        },
        // Copy the contents as it is a folder
        (EntryType::Folder, EntryType::Folder) => move_folder_contents(origin, destination)?,
        // Already exists so will be replaced
        (_, _) => {
            println!(
                "| Entry \"{}\" already exists, do you want to replace (r), skip (s) or cancel (c) at \"{}\"?",
                truncate_path_string(destination),
                truncate_path_string(origin)
            );

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input");

            let trimmed_input = input.trim().to_lowercase();

            if trimmed_input == "r" || trimmed_input == "replace" {
                if crate::delete(destination).is_none() {
                    return Err(format!(
                        "Could not delete \"{}\"",
                        truncate_path_string(destination)
                    ));
                }

                match std::fs::rename(origin, destination) {
                    Ok(_) => (),
                    Err(_) => {
                        println!(
                            "| Unable to move \"{}\" to \"{}\", do you want to cancel? (y)",
                            truncate_path_string(origin),
                            truncate_path_string(destination)
                        );

                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read user input");

                        let trimmed_input = input.trim().to_lowercase();

                        if trimmed_input == "y" || trimmed_input == "yes" {
                            println!("| Cancelling...");
                            return Err("Cancelled".to_string());
                        }
                    }
                }
            } else if trimmed_input == "s" || trimmed_input == "skip" {
                return Ok(());
            } else {
                return Err("Cancelled".to_string());
            }
        }
    }

    Ok(())
}

pub fn move_folder_contents(origin: &Path, destination: &Path) -> Result<(), String> {
    let origin_type = EntryType::get(origin);
    let destination_type = EntryType::get(destination);

    if origin_type != EntryType::Folder {
        return Err(format!(
            "Origin \"{}\" is not a folder",
            truncate_path_string(origin)
        ));
    } else if destination_type != EntryType::Folder {
        return Err(format!(
            "Destination \"{}\" is not a folder",
            truncate_path_string(origin)
        ));
    }

    let entries = match origin.read_dir() {
        Ok(v) => v,
        Err(_) => {
            return Err(format!(
                "Could not read the contents of \"{}\"",
                truncate_path_string(origin)
            ))
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(v) => v,
            Err(_) => continue,
        };

        move_entry_ref(&entry.path(), &destination.join(entry.file_name()))?;
    }

    crate::delete(origin).ok_or(format!(
        "Could not delete \"{}\"",
        truncate_path_string(origin)
    ))?;

    Ok(())
}
