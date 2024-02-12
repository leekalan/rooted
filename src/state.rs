use std::path::PathBuf;

pub struct State {
    pub status: Status,
    pub home: PathBuf,
    pub moving: Moving,
}

pub enum Status {
    None,
    Restarting,
    Quitting,
}

pub enum Moving {
    None,
    Move(PathBuf, PathType, MoveType),
}

pub enum PathType {
    Identity,
    Contents,
}

pub enum MoveType {
    Copy,
    Cut,
}
