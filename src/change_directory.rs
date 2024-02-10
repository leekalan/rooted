use roped::*;

#[allow(dead_code)]
#[derive(Debug, Strand)]
#[strand(state = "EmptyState", action = "action")]
pub struct ChangeDirectory {
    path: String,
}

impl ChangeDirectory {
    fn action(&self, _: &mut EmptyState) -> Result<(), String> {
        todo!()
    }
}
