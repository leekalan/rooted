mod functions;
use functions::*;

use crate::*;

use std::{fs::{self, File}, path::Path};

use self::state::MoveType;

#[derive(Debug)]
pub struct PasteDirectory;

impl Strand for PasteDirectory {
    type State = State;

    fn run(state: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let (input_p, move_info) = if let Some(c) = input.chars().next() {
            if c == '#' {
                (unsafe { input.get_unchecked(1..) }, MoveInfo::Contents)
            } else {
                (input, MoveInfo::Ident)
            }
        } else {
            (input, MoveInfo::Ident)
        };

        let path = input_p
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let destination_dir = get_dir()?.join(std::path::PathBuf::from(path));

        let (copy_dir, move_type) = match &state.moving {
            state::Moving::None => return Err("no directory copied".into()),
            state::Moving::Move(p, m) => (p, m),
        };

        println!("{}", paste_dir(&copy_dir, &destination_dir, move_type, &move_info)?);

        Ok(())
    }
}

fn paste_dir(origin: &Path, destination: &Path, move_type: &MoveType, move_info: &MoveInfo) -> Result<String, String> {
    todo!()
}

enum MoveInfo {
    Ident,
    Contents,
}