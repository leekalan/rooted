use crate::{
    state::{MoveType, Moving},
    *,
};

#[derive(Debug)]
pub struct CopyDirectory;

impl Strand for CopyDirectory {
    type State = State;

    fn run(state: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let path = input
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let new_dir = crate::offset_dir(&std::path::PathBuf::from(path))?;
        println!("Copied \"{}\"", truncate_path_string(&new_dir));
        state.moving = Moving::Move(new_dir, MoveType::Copy);

        Ok(())
    }
}
