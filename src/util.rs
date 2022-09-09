pub fn path_directories() -> Vec<std::path::PathBuf> {
    let mut dirs = vec![];
    if let Some(val) = std::env::var_os("PATH") {
        dirs.extend(std::env::split_paths(&val));
    }
    dirs
}

pub fn is_executable<P: AsRef<std::path::Path>>(path: P) -> bool {
    #[cfg(target_family = "unix")]
        {
            use std::os::unix::prelude::*;
            std::fs::metadata(path)
                .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        }
    #[cfg(target_family = "windows")]
        path.as_ref().is_file()
}