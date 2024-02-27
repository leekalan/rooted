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

pub fn move_item_to(origin: &Path, destination: &Path) -> Result<String, String> {
    let name = origin
        .file_name()
        .ok_or("Unnamed copy directory")?
        .to_string_lossy()
        .to_string();

    let new_des = destination.join(name);
    if new_des.exists() {
        println!("Item already exists, do you want to replace it? (y/n)");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let trimmed_input = input.trim().to_lowercase();

        if trimmed_input == "y" || trimmed_input == "yes" {
            return Ok("Cancelling...".to_string())
        }
        
        //DELETE FUNCTION
        todo!()
    }

    rename_item(origin, &new_des)
}

pub fn move_file_contents_to(origin: &Path, destination: &Path) -> Result<String, String> {
    if destination.exists() {
        todo!()
    }

    rename_item(origin, destination)
}

pub fn move_folder_contents_to(origin: &Path, destination: &Path) -> Result<String, String> {
    if destination.exists() {
        todo!()
    } else {
        rename_item(origin, destination)
    }
}

fn rename_item(origin: &Path, destination: &Path) -> Result<String, String> {
    std::fs::rename(origin, destination).map_err(|_| "Invalid destination")?;
    Ok(format!(
        "Moved item at \"{}\" to \"{}\"",
        truncate_path_string(origin),
        truncate_path_string(destination)
    ))
}
