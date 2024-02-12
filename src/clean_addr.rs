pub fn clean_addr() -> Result<String, String> {
    if let Ok(current_dir) = std::env::current_dir() {
        if let Ok(canonicalized_path) = current_dir.canonicalize() {
            if std::env::set_current_dir(&canonicalized_path).is_err() {
                return Err("!Failed to canonicalize current directory".into());
            }
            Ok(truncate_path_string(&canonicalized_path, 30, 4)?)
        } else {
            Err("!Failed to canonicalize current directory".into())
        }
    } else {
        Err("!Failed to get current directory".into())
    }
}

fn truncate_path_string(
    path: &std::path::Path,
    max_chars: usize,
    max_depth: usize,
) -> Result<String, String> {
    let string = path
        .as_os_str()
        .to_str()
        .ok_or("Could not read path characters")?
        .trim_start_matches(['\\', '?']);
    if string.len() < max_chars {
        return Ok(string.into());
    }
    let components: Vec<_> = path.components().collect();
    if components.len() < max_depth {
        return Ok(string.into());
    }

    let mut string = String::new();

    let mut index = 1usize;
    for component in components.get(components.len() - max_depth..).unwrap() {
        let s = component
            .as_os_str()
            .to_str()
            .ok_or("Could not read path characters")?;
        if string.len() + s.len() >= max_chars {
            break;
        }
        string.push('\\');
        string.push_str(s);
        if index >= max_depth {
            break;
        }
        index += 1;
    }

    Ok(format!("...{}", string))
}
