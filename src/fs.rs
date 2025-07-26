pub fn get_base_path(file_path: &str) -> Option<&str> {
    let path = std::path::Path::new(file_path);
    path.parent().map(|p| p.to_str()).flatten()
}

pub fn get_file_extension(file_path: &str) -> Option<&str> {
    let path = std::path::Path::new(file_path);
    path.extension().map(|s| s.to_str()).flatten()
}

pub fn create_garbage_base(path: &str) {
    // TODO: handle error
    if let Ok(true) = std::fs::exists(path) {
        info!("directory `{}` already exists", path);
    } else {
        info!("directory `{}` created", path);
        let _ = std::fs::create_dir(path);
    }
}

pub fn get_file_stem(file_path: &str) -> Option<&str> {
    let path = std::path::Path::new(file_path);
    path.file_stem().map(|f| f.to_str()).flatten()
}

pub fn get_garbage_base(file_path: &str) -> Option<String> {
    const GARBAGE_PATH_NAME: &str = ".build";

    let path = std::path::Path::new(file_path);
    path.parent()
        .map(|p| {
            p.join(GARBAGE_PATH_NAME)
                .into_os_string()
                .into_string()
                .ok()
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_path() {
        assert_eq!(get_base_path("foot.txt"), Some(""));
        assert_eq!(get_base_path("base/foot.txt"), Some("base"));
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("foot.txt"), Some("txt"));
        assert_eq!(get_file_extension("base/foot.hs"), Some("hs"));
    }

    #[test]
    fn test_get_garbage_base() {
        assert_eq!(get_garbage_base("foot.txt"), Some(".build".to_string()));
        assert_eq!(
            get_garbage_base("base/foot.hs"),
            Some("base/.build".to_string())
        );
    }
}
