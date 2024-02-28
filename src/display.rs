use crate::*;

use castr::cast::Castable;
use parsr::parse::Parse;

use std::path::Path;

use self::state::DisplayOption;

#[derive(Debug)]
pub struct DisplayDirectory;

impl Strand for DisplayDirectory {
    type State = State;

    fn run(state: &mut Self::State, input: &str, ws: &[char]) -> Result<(), String> {
        let parse_result = input.parse_one_arg(ws);

        let (input_p, depth): (_, usize) = match parse_result.parsed.cast_to() {
            Some(v) => (parse_result.excess.unwrap_or(""), v),
            None => (input, state.display.default_depth),
        };

        let path = input_p
            .trim_start_matches(ws)
            .trim_end_matches(ws)
            .replace('~', "..");
        let new_dir = crate::offset_dir(&std::path::PathBuf::from(path))?;
        println!(
            "{}",
            display(&new_dir, depth - 1, &state.display.display_style)?
        );

        Ok(())
    }
}

fn display(path: &Path, depth: usize, display: &DisplayOption) -> Result<String, String> {
    if !is_valid(path) {
        return Err("Invalid path".into());
    }

    let mut accumulator = String::from('\n');

    accumulator += &path
        .components()
        .last()
        .map_or("<empty>".to_string(), |component| {
            component.as_os_str().to_string_lossy().to_string()
        });
    accumulator.push('\n');

    let mut display_info = DisplayInfo {
        container: DisplayContainer {
            name: String::new(),
            display_type: DisplayType::Other,
        },
        offset: Vec::with_capacity(depth),
    };

    accumulator += &display_sub(path, &mut display_info, depth, display);

    Ok(accumulator)
}

fn display_sub(
    path: &Path,
    display_info: &mut DisplayInfo,
    depth: usize,
    display: &DisplayOption,
) -> String {
    let mut accumulator = String::new();

    let entries: Vec<Result<std::fs::DirEntry, _>> = match std::fs::read_dir(path) {
        Ok(v) => v.collect(),
        Err(_) => {
            display_info.container = DisplayContainer {
                name: format!("Corrupted Directory \"{}\"", path.to_string_lossy()),
                display_type: DisplayType::CoreErr,
            };

            accumulator += &display_info.read(display);
            accumulator.push('\n');
            return accumulator;
        }
    };

    for (i, entry_w) in entries.iter().enumerate() {
        let entry = match entry_w.as_ref() {
            Ok(v) => v,
            Err(_) => {
                display_info.container = DisplayContainer {
                    name: "<invalid_entry>".to_owned(),
                    display_type: DisplayType::Err,
                };

                accumulator += &display_info.read(display);
                accumulator.push('\n');
                continue;
            }
        };

        let file_type = entry.file_type();

        let is_final_entry = entries.len() - 1 == i;

        let file_name = entry.file_name().to_string_lossy().to_string();

        if let Ok(file_type) = file_type {
            if file_type.is_dir() {
                display_info.container = DisplayContainer {
                    name: file_name.clone(),
                    display_type: DisplayType::Folder,
                };

                accumulator += &display_info.read(display);
                accumulator.push('\n');

                if depth > 0 {
                    let new_path = path.join(file_name);

                    let offset = if is_final_entry {
                        DisplaySpacing::Gap
                    } else {
                        DisplaySpacing::Pipe
                    };
                    display_info.offset.push(offset);

                    accumulator += &display_sub(&new_path, display_info, depth - 1, display);

                    display_info.offset.pop();
                }
            } else if file_type.is_file() {
                let display_type = if is_final_entry {
                    DisplayType::End
                } else {
                    DisplayType::Item
                };

                display_info.container = DisplayContainer {
                    name: file_name,
                    display_type,
                };

                accumulator += &display_info.read(display);
                accumulator.push('\n');
            } else {
                display_info.container = DisplayContainer {
                    name: file_name,
                    display_type: DisplayType::Other,
                };

                accumulator += &display_info.read(display);
                accumulator.push('\n');
            }
        } else {
            display_info.container = DisplayContainer {
                name: file_name,
                display_type: DisplayType::Err,
            };

            accumulator += &display_info.read(display);
            accumulator.push('\n');
        }
    }

    accumulator
}

enum DisplayType {
    Folder,
    Item,
    Other,
    Err,
    CoreErr,
    End,
}
impl DisplayType {
    fn read(&self, display: &state::DisplayOption) -> &str {
        match display {
            state::DisplayOption::Clean => match self {
                DisplayType::Folder => "+ ",
                DisplayType::Item => "| ",
                DisplayType::Other => "? ",
                DisplayType::Err => "! ",
                DisplayType::CoreErr => "!!",
                DisplayType::End => "| ",
            },
            state::DisplayOption::Detailed => match self {
                DisplayType::Folder => "+-",
                DisplayType::Item => "|-",
                DisplayType::Other => "?-",
                DisplayType::Err => "!-",
                DisplayType::CoreErr => "!!",
                DisplayType::End => "`-",
            },
        }
    }
}

struct DisplayContainer {
    name: String,
    display_type: DisplayType,
}
impl DisplayContainer {
    fn read(&self, display: &state::DisplayOption) -> String {
        let mut concat = self.display_type.read(display).to_owned();
        concat += &self.name;
        concat
    }
}

enum DisplaySpacing {
    Pipe,
    Gap,
}
impl DisplaySpacing {
    fn read(&self) -> &str {
        match self {
            DisplaySpacing::Pipe => "| ",
            DisplaySpacing::Gap => "  ",
        }
    }
}

struct DisplayInfo {
    container: DisplayContainer,
    offset: Vec<DisplaySpacing>,
}
impl DisplayInfo {
    fn read(&self, display: &state::DisplayOption) -> String {
        let preface = self
            .offset
            .iter()
            .fold(String::with_capacity(self.offset.len() * 2), |acc, x| {
                acc + x.read()
            });
        preface + &self.container.read(display)
    }
}
