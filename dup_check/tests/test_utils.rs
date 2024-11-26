use dup_check::utils;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create a temporary file with specific content
fn create_temp_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
    let file_path = dir.path().join(name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file_path
}

#[test]
fn test_calculate_hash_identical_files() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create two identical files
    let content = b"Hello, World!";
    let file1 = create_temp_file(&temp_dir, "file1.txt", content);
    let file2 = create_temp_file(&temp_dir, "file2.txt", content);
    
    let hash1 = utils::calculate_hash(&file1).unwrap();
    let hash2 = utils::calculate_hash(&file2).unwrap();
    
    assert_eq!(hash1, hash2, "Hashes of identical files should match");
}

#[test]
fn test_calculate_hash_different_files() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create two different files
    let file1 = create_temp_file(&temp_dir, "file1.txt", b"Hello, World!");
    let file2 = create_temp_file(&temp_dir, "file2.txt", b"Different content");
    
    let hash1 = utils::calculate_hash(&file1).unwrap();
    let hash2 = utils::calculate_hash(&file2).unwrap();
    
    assert_ne!(hash1, hash2, "Hashes of different files should not match");
}

#[test]
fn test_calculate_hash_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file = create_temp_file(&temp_dir, "empty.txt", b"");
    
    let hash = utils::calculate_hash(&file).unwrap();
    assert!(!hash.is_empty(), "Hash of empty file should not be empty");
}

#[test]
fn test_is_hidden_file() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a regular file
    let regular_file = create_temp_file(&temp_dir, "regular.txt", b"");
    assert!(!utils::is_hidden(&regular_file), "Regular file should not be hidden");
    
    // Create a hidden file (platform-dependent)
    #[cfg(unix)]
    {
        let hidden_file = create_temp_file(&temp_dir, ".hidden", b"");
        assert!(utils::is_hidden(&hidden_file), "File starting with dot should be hidden on Unix");
    }
}

#[test]
fn test_calculate_hash_large_file() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a 1MB file with repeating pattern
    let mut large_content = Vec::with_capacity(1024 * 1024);
    for _ in 0..(1024 * 1024 / 4) {
        large_content.extend_from_slice(b"test");
    }
    
    let file = create_temp_file(&temp_dir, "large.txt", &large_content);
    let hash = utils::calculate_hash(&file).unwrap();
    assert!(!hash.is_empty(), "Hash of large file should not be empty");
}

#[test]
fn test_calculate_hash_binary_file() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a binary file with null bytes
    let binary_content: Vec<u8> = (0..255).collect();
    let file = create_temp_file(&temp_dir, "binary.bin", &binary_content);
    
    let hash = utils::calculate_hash(&file).unwrap();
    assert!(!hash.is_empty(), "Hash of binary file should not be empty");
}

#[test]
fn test_calculate_hash_nonexistent_file() {
    let nonexistent = PathBuf::from("/nonexistent/file.txt");
    let result = utils::calculate_hash(&nonexistent);
    assert!(result.is_err(), "Expected an error for nonexistent file");
    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("cannot find") || err_msg.contains("No such file"),
        "Unexpected error message: {}",
        err_msg
    );
}
