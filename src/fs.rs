pub struct Files<'prog> {
    pub output_path: std::ffi::OsString,
    pub object_path: std::ffi::OsString,
    pub build_path: &'prog str,
}

impl<'prog> Files<'prog> {
    pub fn new(output_file: &'prog str) -> Self {
        // TODO: handle this error
        let garbage_path = get_garbage_base(output_file).unwrap();

        create_garbage_base(&garbage_path);

        let output_stem = get_file_stem(output_file).expect("invalid o path");
        let g_path = std::path::Path::new(&garbage_path);
        let output_path = g_path.join(format!("{output_stem}.s")).into_os_string();
        let object_path = g_path.join(format!("{output_stem}.o")).into_os_string();

        Self {
            output_path,
            object_path,
            build_path: output_file,
        }
    }
}

pub fn get_base_path(file_path: &str) -> Option<&str> {
    let path = std::path::Path::new(file_path);
    path.parent().map(|p| p.to_str()).flatten()
}

pub fn get_file_extension(file_path: &str) -> &str {
    if let Some(last_dot) = file_path.rfind(".") {
        // SAFETY: this is safe because
        // - starting index is 0 and does not exceed last index
        // - last_dot does not exceed last index
        // - file_path is a valid UTF-8 str
        unsafe { file_path.get_unchecked(last_dot..) }
    } else {
        ""
    }
}

pub fn strip_extension(file_path: &str) -> &str {
    let file_extension = get_file_extension(file_path);
    // SAFETY: this is safe because
    // - starting index is 0 and does not exceed last index
    // - last_dot does not exceed last index
    // - file_path is a valid UTF-8 str(file_path);
    unsafe { file_path.get_unchecked(0..(file_path.len() - file_extension.len())) }
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
        assert_eq!(get_file_extension("foot.txt"), ".txt");
        assert_eq!(get_file_extension("base/foot.hs"), ".hs");
    }

    #[test]
    fn test_strip_file_extension() {
        assert_eq!(strip_extension("foot.txt"), "foot");
        assert_eq!(strip_extension("base/foot.hs"), "base/foot");
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
