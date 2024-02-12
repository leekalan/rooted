use roped::*;

use crate::State;

#[allow(dead_code)]
#[derive(Debug, Strand)]
#[strand(state = "State", action = "action")]
pub struct ChangeDirectory {
    path: String,
}

impl ChangeDirectory {
    fn action(&self, _: &mut State) -> Result<(), String> {
        todo!()
    }
}
