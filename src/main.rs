use roped::*;

mod change_directory;
mod sys;

use change_directory::ChangeDirectory;
use sys::Sys;

#[allow(dead_code)]
#[derive(Debug, Bundle)]
#[bundle(state = "EmptyState")]
enum Container {
    #[bundle(prefix = "$")]
    Sys(Sys),
    #[bundle(prefix = "@")]
    At(ChangeDirectory),
    #[bundle(name = "cd")]
    CD(ChangeDirectory),
}

fn main() {
    loop {
        let clean_addr = "\\TEMP";
        let prompt = format!("{} > ", clean_addr);
        run_console::<Container, EmptyState>(
            &mut EmptyState,
            Some(&prompt),
            ". ".into(),
            "!".into(),
            &[' '],
            &['\n'],
        )
    }
}
