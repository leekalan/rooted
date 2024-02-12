use roped::*;

mod boot;
mod change_directory;
mod state;
mod sys;

use boot::boot;
use change_directory::ChangeDirectory;
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

fn main() {
    if let Err(err) = boot() {
        println!("{}", err);
        std::process::exit(1);
    }

    let mut state = State {
        moving: state::Moving::None,
    };

    loop {
        let clean_addr = "\\TEMP";
        let prompt = format!("{} > ", clean_addr);
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
