use std::env;

use roped::*;

mod boot;
mod change_directory;
mod clean_addr;
mod copy;
mod cut;
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
use dir::*;
use display::DisplayDirectory;
use paste::PasteDirectory;
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
