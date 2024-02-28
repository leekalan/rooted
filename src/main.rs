use std::env;

use roped::*;

mod boot;
mod change_directory;
mod clean_addr;
mod copy;
mod cut;
mod delete;
mod dir;
mod display;
mod paste;
mod state;
mod sys;

use boot::*;
use change_directory::ChangeDirectory;
use clean_addr::*;
use copy::CopyDirectory;
use cut::CutDirectory;
use delete::*;
use dir::*;
use display::DisplayDirectory;
use paste::PasteDirectory;
use state::DisplayState;
use state::State;
use state::Status;
use sys::Sys;

#[allow(dead_code)]
#[derive(Debug, Bundle)]
#[bundle(state = "State")]
enum Container {
    #[bundle(prefix = "$")]
    Dollar(Sys),
    #[bundle(name = "sys")]
    Sys(Sys),

    #[bundle(prefix = "@")]
    At(ChangeDirectory),
    #[bundle(name = "cd")]
    CD(ChangeDirectory),

    #[bundle(prefix = "#")]
    Hash(DisplayDirectory),
    #[bundle(name = "list")]
    List(DisplayDirectory),

    #[bundle(prefix = "&")]
    And(CopyDirectory),
    #[bundle(name = "copy")]
    Copy(CopyDirectory),

    #[bundle(prefix = "^")]
    Move(CutDirectory),
    #[bundle(name = "cut")]
    Cut(CutDirectory),

    #[bundle(prefix = "*")]
    Deref(PasteDirectory),
    #[bundle(name = "paste")]
    Paste(PasteDirectory),
}

fn begin() -> State {
    if let Err(err) = boot() {
        println!("{}", err);
        std::process::exit(1);
    }

    State {
        status: Status::None,
        home: env::current_dir().unwrap(),
        moving: state::Moving::None,
        display: DisplayState {
            default_depth: 1,
            display_style: state::DisplayOption::Clean,
        },
    }
}

fn main() {
    let mut state = begin();

    loop {
        let addr = match clean_addr() {
            Ok(v) => v,
            Err(err) => {
                println!("{}\nRestarting...", err);
                state = begin();
                continue;
            }
        };

        let prompt = format!("{} > ", addr);
        run_console::<Container, State>(
            &mut state,
            Some(&prompt),
            ". ".into(),
            "!".into(),
            &[' '],
            &[';', '\n'],
        );

        match state.status {
            Status::None => (),
            Status::Restarting => {
                println!("Restarting...");
                state = begin();
            }
            Status::Quitting => {
                println!("Exiting the process shortly...");
                std::process::exit(0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //
    // TestDir
    // +-FolderA
    // | |-Item1 (contents: ITEMA1)
    // | `-Item2 (contents: ITEMA2)
    // +-FolderB
    // | |-Item1 (contents: ITEMB1)
    // | `-Item2 (contents: ITEMB2)
    // |-ItemA.txt (contents: ITEMA)
    // |-ItemB.txt (contents: ITEMB)
    // |-Item1.txt (contents: ITEM1)
    // `-Item2.txt (contents: ITEM2)
    //

    #[test]
    fn rebuild_test_directory() {
        let origin = std::path::PathBuf::from("C:\\Users\\kalan\\test_template");
        let destination = std::path::PathBuf::from("C:\\Users\\kalan\\test_instance");

        let _ = std::fs::remove_dir_all(&destination);

        copy_directory(&origin, &destination)
    }
    #[allow(dead_code)]
    fn copy_directory(src: &std::path::Path, dest: &std::path::Path) {
        std::fs::create_dir_all(dest).unwrap();

        for entry in std::fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let entry_type = entry.file_type().unwrap();

            let entry_dest = dest.join(entry.file_name());

            if entry_type.is_dir() {
                copy_directory(&entry.path(), &entry_dest);
            } else {
                std::fs::copy(&entry.path(), &entry_dest).unwrap();
            }
        }
    }
}
