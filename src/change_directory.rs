use roped::*;

use crate::State;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ChangeDirectory {
    path: String,
}

impl Strand for ChangeDirectory {
    type State = State;

    fn run(_: &mut Self::State, input: &str, _: &[char]) -> Result<(), String> {
        let new_dir = crate::offset_dir(&std::path::PathBuf::from(input))?;
        crate::set_dir(&new_dir)
    }
}
