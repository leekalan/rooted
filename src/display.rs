use std::path::Path;

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
        offset: 0,
        position_offset: 0,
    };

    accumulator += &display_sub(path, display_info, depth)?;

    Ok(accumulator)
}

fn display_sub(path: &Path, display_info: DisplayInfo, depth: usize) -> Result<String, String> {
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

struct DisplayInfo {
    container: Option<DisplayContainer>,
    offset: usize,
    position_offset: usize,
}
impl DisplayInfo {
    fn read(&self) -> String {
        let preface = "| ".repeat(self.offset) + &"  ".repeat(self.position_offset);
        let result = match self.container.as_ref() {
            Some(container) => preface + &container.read(),
            None => preface,
        };
        result
    }
}
