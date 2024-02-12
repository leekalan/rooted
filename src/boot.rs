use std::io::{self, Write};

pub fn boot() -> Result<(), String> {
    if std::env::set_current_dir(std::path::Path::new(r"C:\")).is_err() {
        println!("!Unable to set root directory to C:\\");

        loop {
            println!("Enter a valid directory to begin the program on: ");
            if io::stdout().flush().is_err() {
                return Err("!!!Unable to use console".into())
            }

            let mut user_input = String::new();
            if io::stdin().read_line(&mut user_input).is_err() {
                println!("!Unable to read console\nRetrying...");
                continue
            }

            let user_input = user_input.trim();

            if let Ok(path) = std::path::Path::new(user_input).canonicalize() {
                if std::env::set_current_dir(&path).is_err() {
                    println!("!Unable to set as root directory\nRetrying...");
                    continue
                } else {
                    println!("Succesfully set \"{}\" as root directory", user_input);
                    
                    //print!("Would you like to set this as your default directory?");
                    
                    return Ok(())
                }
            } else {
                println!("!Invalid path\nRetrying...");
                continue
            }
        }
    }

    Ok(())
}
