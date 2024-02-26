use std::{fs::FileType, path::Path};

use crate::is_valid;

//
// Folder1
// +-Folder2
// | |-File 1
// | |
// | `-File 2
// |
// |-File 1
// |
// +-Folder 3
//   |-File 1
//   |
//   `-File 2
//

fn list_elements(path: &Path) -> Vec<ElementType> {
    todo!()
}
enum ElementType {
    File,
    Folder,
}
struct Element {
    name: String,
    element_type: ElementType,
}

fn display(path: &Path, depth: usize) -> Result<String, String> {
    if !is_valid(path) {
        return Err("Invalid path".into());
    }

    let original_dir = crate::dir::get_dir()?;

    let mut accumulator: String = path
        .components()
        .last()
        .map_or("<empty>", |component| {
            component.as_os_str().to_str().unwrap_or("<empty>")
        })
        .to_owned();
    accumulator.push('\n');

    let mut display_info = DisplayInfo {
        container: None,
        offset: Vec::with_capacity(depth),
    };

    accumulator += &display_sub(path, &mut display_info, depth)?;

    Ok(accumulator)
}

fn display_sub(
    path: &Path,
    display_info: &mut DisplayInfo,
    depth: usize,
) -> Result<String, String> {
    let mut entries: Vec<Result<std::fs::DirEntry, _>> = std::fs::read_dir(path).map_err(|_| "Could not read directory")?.collect();
    for (i, entry_w) in entries.iter().enumerate() {
        let entry = entry_w.as_ref().map_err(|_| "Could not read entry")?;

        let file_type = entry.file_type().map_err(|_| "Could not read file type")?;

        let final_element = entries.len() - 1 == i;

        if file_type.is_file() {
        } else if file_type.is_dir() {

        } else {

        }
    }

    todo!()
}

enum DisplayType {
    Folder,
    Item,
    End,
}
impl DisplayType {
    fn read(&self) -> &str {
        match self {
            DisplayType::Folder => "+-",
            DisplayType::Item => "|-",
            DisplayType::End => "`-",
        }
    }
}

struct DisplayContainer {
    name: String,
    display_type: DisplayType,
}
impl DisplayContainer {
    fn read(&self) -> String {
        let mut concat = self.display_type.read().to_owned();
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
    container: Option<DisplayContainer>,
    offset: Vec<DisplaySpacing>,
}
impl DisplayInfo {
    fn read(&self) -> String {
        let preface = self
            .offset
            .iter()
            .fold(String::with_capacity(self.offset.len() * 2), |acc, x| {
                acc + x.read()
            });
        let result = match self.container.as_ref() {
            Some(container) => preface + &container.read(),
            None => preface,
        };
        result
    }
}
