use roped::*;

mod sys;
mod change_directory;

use sys::Sys;
use change_directory::ChangeDirectory;

#[allow(dead_code)]
#[derive(Debug, Bundle)]
#[bundle(state = "EmptyState")]
enum Container {
    #[bundle(prefix = "$")]
    Sys(Sys),
    #[bundle(prefix = "@")]
    ChangeDirectory(ChangeDirectory),
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
