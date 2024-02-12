use roped::*;

use crate::State;

#[allow(dead_code)]
#[derive(Debug, Bundle)]
#[bundle(state = "State")]
pub enum Sys {
    #[bundle(name = "quit")]
    Quit(Quit),
    #[bundle(name = "stop")]
    Stop(Quit),
}

#[derive(Debug)]
pub struct Quit;

impl Strand for Quit {
    type State = State;

    fn run(_: &mut Self::State, _: &str, _: &[char]) -> Result<(), String> {
        println!("Exiting the process shortly...");
        std::process::exit(0);
    }
}
