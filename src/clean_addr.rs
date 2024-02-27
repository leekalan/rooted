pub fn clean_addr() -> Result<String, String> {
    if let Ok(current_dir) = std::env::current_dir() {
        let canonicalized_path = canonicalize_path(&current_dir)?;
        if std::env::set_current_dir(&canonicalized_path).is_err() {
            return Err("Failed to canonicalize current directory".into());
        }
        Ok(truncate_path_string(&canonicalized_path))
    } else {
        Err("Failed to get current directory".into())
    }
}

pub fn canonicalize_path(path: &std::path::Path) -> Result<std::path::PathBuf, String> {
    path.canonicalize()
        .map_err(|_| "Failed to canonicalize current directory".into())
}

pub fn truncate_path_string(path: &std::path::Path) -> String {
    truncate_path_string_core(path, 30, 4)
}

fn truncate_path_string_core(path: &std::path::Path, max_chars: usize, max_depth: usize) -> String {
    let temp = path.as_os_str().to_string_lossy().to_string();
    let string = temp.trim_start_matches(['\\', '?']);
    if string.len() < max_chars {
        return string.into();
    }
    let components: Vec<_> = path.components().collect();
    if components.len() < max_depth {
        return string.into();
    }

    let mut string = String::new();

    let mut index = 1usize;
    for component in components.get(components.len() - max_depth..).unwrap() {
        let s = component.as_os_str().to_string_lossy().to_string();
        if string.len() + s.len() >= max_chars {
            break;
        }
        string.push('\\');
        string.push_str(&s);
        if index >= max_depth {
            break;
        }
        index += 1;
    }

    format!("..{}", string)
}
