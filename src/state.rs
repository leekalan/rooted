use std::path::PathBuf;

pub struct State {
    pub moving: Moving,
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
