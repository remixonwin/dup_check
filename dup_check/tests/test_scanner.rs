use dup_check::scanner::Scanner;
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

/// Helper function to create a directory structure with test files
fn create_test_directory() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    
    // Create some duplicate files
    let content1 = b"Hello, World!";
    create_temp_file(&temp_dir, "file1.txt", content1);
    create_temp_file(&temp_dir, "file2.txt", content1);
    
    // Create a unique file
    create_temp_file(&temp_dir, "unique.txt", b"Unique content");
    
    // Create a subdirectory with duplicates
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();
    create_temp_file(&temp_dir, "subdir/file3.txt", content1);
    
    // Create empty files
    create_temp_file(&temp_dir, "empty1.txt", b"");
    create_temp_file(&temp_dir, "empty2.txt", b"");
    
    temp_dir
}

#[test]
fn test_scanner_find_duplicates() {
    let temp_dir = create_test_directory();
    
    let scanner = Scanner::new(false, None, None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    
    // Should find two groups of duplicates (non-empty and empty files)
    assert_eq!(duplicates.len(), 2, "Should find two groups of duplicates");
    
    // Verify duplicate groups
    for (_, files) in duplicates.iter() {
        match files[0].size {
            0 => assert_eq!(files.len(), 2, "Should find 2 empty files"),
            13 => assert_eq!(files.len(), 3, "Should find 3 duplicate 'Hello, World!' files"),
            _ => panic!("Unexpected file size in duplicates"),
        }
    }
}

#[test]
fn test_scanner_with_size_filter() {
    let temp_dir = create_test_directory();
    
    // Test minimum size filter (exclude empty files)
    let scanner = Scanner::new(false, Some(1), None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    assert_eq!(duplicates.len(), 1, "Should only find non-empty duplicates");
    
    // Test maximum size filter (exclude non-empty files)
    let scanner = Scanner::new(false, None, Some(1)).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    assert_eq!(duplicates.len(), 1, "Should only find empty duplicates");
}

#[test]
fn test_scanner_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    
    let scanner = Scanner::new(false, None, None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    assert!(duplicates.is_empty(), "Empty directory should have no duplicates");
}

#[test]
fn test_scanner_single_files() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create several unique files
    create_temp_file(&temp_dir, "file1.txt", b"Content 1");
    create_temp_file(&temp_dir, "file2.txt", b"Content 2");
    create_temp_file(&temp_dir, "file3.txt", b"Content 3");
    
    let scanner = Scanner::new(false, None, None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    assert!(duplicates.is_empty(), "No duplicates should be found");
}

#[test]
fn test_scanner_with_hidden_files() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create duplicate content in both hidden and non-hidden files
    let content = b"Duplicate content";
    create_temp_file(&temp_dir, "visible1.txt", content);
    create_temp_file(&temp_dir, "visible2.txt", content);
    
    // Create hidden files with proper Windows hidden attribute
    let hidden1 = create_temp_file(&temp_dir, "hidden1.txt", content);
    let hidden2 = create_temp_file(&temp_dir, "hidden2.txt", content);
    
    // Set hidden attribute on Windows
    #[cfg(windows)]
    {
        use windows::Win32::Storage::FileSystem::{SetFileAttributesW, FILE_ATTRIBUTE_HIDDEN};
        for path in &[hidden1, hidden2] {
            let wide_path: Vec<u16> = path.to_string_lossy()
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            unsafe {
                SetFileAttributesW(
                    windows::core::PCWSTR::from_raw(wide_path.as_ptr()),
                    FILE_ATTRIBUTE_HIDDEN
                );
            }
        }
    }
    
    let scanner = Scanner::new(false, None, None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    
    // Should only find the visible duplicates
    assert_eq!(duplicates.len(), 1, "Should find one group of duplicates");
    for (_, files) in duplicates {
        assert_eq!(files.len(), 2, "Should only find visible duplicate files");
        for file in files {
            assert!(!utils::is_hidden(&file.path), "Hidden files should be excluded");
        }
    }
}

#[test]
fn test_scanner_large_files() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create large duplicate files (1MB each)
    let mut large_content = Vec::with_capacity(1024 * 1024);
    for _ in 0..(1024 * 1024 / 4) {
        large_content.extend_from_slice(b"test");
    }
    
    create_temp_file(&temp_dir, "large1.txt", &large_content);
    create_temp_file(&temp_dir, "large2.txt", &large_content);
    
    let scanner = Scanner::new(false, None, None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    
    assert_eq!(duplicates.len(), 1, "Should find one group of duplicates");
    for (_, files) in duplicates {
        assert_eq!(files.len(), 2, "Should find two duplicate large files");
        assert_eq!(files[0].size, 1024 * 1024, "Files should be 1MB in size");
    }
}
