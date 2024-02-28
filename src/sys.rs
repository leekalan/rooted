use roped::*;

use crate::{state::DisplayOption, State, Status};

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

    #[bundle(name = "display")]
    Display(DisplayConfig),
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

#[allow(dead_code)]
#[derive(Debug, Bundle)]
#[bundle(state = "State")]
pub enum DisplayConfig {
    #[bundle(prefix = "*")]
    Count(Depth),
    #[bundle(name = "depth")]
    Depth(Depth),

    #[bundle(prefix = "%")]
    Variant(Display),
    #[bundle(name = "style")]
    Display(Display),
}

#[derive(Debug, Strand)]
#[strand(state = "State", action = "action")]
pub struct Depth {
    depth: usize,
}
impl Depth {
    fn action(self, state: &mut State) -> Result<(), String> {
        state.display.default_depth = self.depth;
        println!("Changed cfg.display.default_depth to {}", self.depth);
        Ok(())
    }
}

#[derive(Debug, Strand)]
#[strand(state = "State", action = "action")]
pub struct Display {
    display_style: DisplayOption,
}
impl Display {
    fn action(self, state: &mut State) -> Result<(), String> {
        state.display.display_style = self.display_style;
        println!(
            "Changed cfg.display.display_style to {}",
            self.display_style
        );
        Ok(())
    }
}
