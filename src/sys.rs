use roped::*;

use crate::{State, Status};

#[allow(dead_code)]
#[derive(Debug, Bundle)]
#[bundle(state = "State")]
pub enum Sys {
    #[bundle(name = "restart")]
    Restart(Restart),
    #[bundle(name = "quit")]
    Quit(Quit),
    #[bundle(name = "stop")]
    Stop(Quit),
}

#[derive(Debug)]
pub struct Restart;

impl Strand for Restart {
    type State = State;

    fn run(state: &mut Self::State, _: &str, _: &[char]) -> Result<(), String> {
        state.status = Status::Restarting;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Quit;

impl Strand for Quit {
    type State = State;

    fn run(state: &mut Self::State, _: &str, _: &[char]) -> Result<(), String> {
        state.status = Status::Quitting;
        Ok(())
    }
}
