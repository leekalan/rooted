use std::{path::PathBuf, str::FromStr};

pub struct State {
    pub status: Status,
    pub home: PathBuf,
    pub moving: Moving,
    pub display: DisplayState,
    pub file: Option<std::fs::File>,
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
    pub display_style: DisplayOption,
}
#[derive(Debug, Clone, Copy)]
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
impl std::fmt::Display for DisplayOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayOption::Clean => f.write_str("Clean"),
            DisplayOption::Detailed => f.write_str("Fancy"),
        }
    }
}
