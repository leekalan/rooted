use parsr::matcher::{MatchContainer, MatcherStart};
use roped::Strand;

use crate::state::State;

pub enum FileViewType {
    Binary,
    Hexadecimal,
    String,
}

#[derive(Debug)]
pub struct WriteFile;

impl Strand for WriteFile {
    type State = State;

    fn run(_: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let (input_p, view) = if let Some(r) = input.remove_starts_with(MatchContainer::Ident("0b")) {
            (r, FileViewType::Binary)
        } else if let Some(r) = input.remove_starts_with(MatchContainer::Ident("0x")) {
            (r, FileViewType::Hexadecimal)
        } else if let Some(r) = input.remove_starts_with(MatchContainer::Ident("0s")) {
            (r, FileViewType::String)
        } else {
            (input, FileViewType::String)
        };
        
        //TODO CONVERTION AND WRITE TO FILE USING CONVERTR CRATE
        todo!()
    }
}
