use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub hash: Option<String>,
}

impl FileInfo {
    pub fn new(path: PathBuf, size: u64) -> Self {
        FileInfo {
            path,
            size,
            hash: None,
        }
    }

    pub fn with_hash(path: PathBuf, size: u64, hash: String) -> Self {
        FileInfo {
            path,
            size,
            hash: Some(hash),
        }
    }
}
