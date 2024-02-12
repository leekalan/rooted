use std::env;

use roped::*;

mod boot;
mod change_directory;
mod clean_addr;
mod state;
mod sys;

use boot::boot;
use change_directory::ChangeDirectory;
use clean_addr::clean_addr;
use state::State;
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
}

fn begin() -> State {
    if let Err(err) = boot() {
        println!("{}", err);
        std::process::exit(1);
    }

    State {
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
                println!("{}\nReloading", err);
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
            &['\n'],
        )
    }
}
