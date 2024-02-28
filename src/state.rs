use std::{path::PathBuf, str::FromStr};

pub struct State {
    pub status: Status,
    pub home: PathBuf,
    pub moving: Moving,
    pub display: DisplayState,
}

pub enum Status {
    None,
    Restarting,
    Quitting,
}

pub enum Moving {
    None,
    Move(PathBuf, MoveType),
}
pub enum MoveType {
    Copy,
    Cut,
}

pub struct DisplayState {
    pub default_depth: usize,
    pub display_type: DisplayOption,
}
#[derive(Debug)]
pub enum DisplayOption {
    Clean,
    Detailed,
}
impl FromStr for DisplayOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "C" | "clean" | "Clean" => Ok(DisplayOption::Clean),
            "f" | "F" | "fancy" | "Fancy" => Ok(DisplayOption::Detailed),
            _ => Err(()),
        }
    }
}