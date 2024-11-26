//! DupCheck - A safe and efficient duplicate file finder
//! 
//! This library provides functionality to find and manage duplicate files
//! in a safe and efficient manner, using parallel processing and caching.
//! 
//! # Features
//! 
//! - Fast file scanning with SHA-256 hash comparison
//! - SQLite-based caching for improved performance
//! - Parallel processing using rayon
//! - Safe file operations with Windows API integration
//! - Progress tracking and detailed statistics
//! 
//! # Version
//! 
//! Current version: 0.1.0

pub mod cache;
pub mod cli;
pub mod file_info;
pub mod scanner;
pub mod ui;
pub mod utils;

pub use cache::Cache;
pub use cli::Args;
pub use file_info::FileInfo;
pub use scanner::Scanner;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_duplicate_detection() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let content = b"test content";

        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("file2.txt");
        
        File::create(&file1_path)?.write_all(content)?;
        File::create(&file2_path)?.write_all(content)?;

        let scanner = Scanner::new(false, None, None)?;
        let duplicates = scanner.find_duplicates(temp_dir.path().to_str().unwrap())?;

        assert_eq!(duplicates.len(), 1); // One group of duplicates
        assert_eq!(duplicates.values().next().unwrap().len(), 2); // Two files in the group

        Ok(())
    }
}
