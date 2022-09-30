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

pub mod u128_dec_format {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
        where
            D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod u64_dec_format {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
        where
            D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}
