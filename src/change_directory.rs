use roped::*;

use crate::State;

#[derive(Debug)]
pub struct ChangeDirectory;

impl Strand for ChangeDirectory {
    type State = State;

    fn run(_: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let path = input
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let new_dir = crate::offset_dir(&std::path::PathBuf::from(path))?;
        crate::set_dir(&new_dir)
    }
}
