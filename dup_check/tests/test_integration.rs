use dup_check::{cache::Cache, scanner::Scanner};
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
fn test_full_duplicate_scan_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Create a complex directory structure with various duplicates
    let content1 = b"Hello, World!";
    let content2 = b"Different content";

    // Create root level duplicates
    create_temp_file(&temp_dir, "file1.txt", content1);
    create_temp_file(&temp_dir, "file2.txt", content1);
    create_temp_file(&temp_dir, "unique1.txt", content2);

    // Create nested directory structure
    let subdir1 = temp_dir.path().join("subdir1");
    let subdir2 = temp_dir.path().join("subdir2");
    fs::create_dir(&subdir1).unwrap();
    fs::create_dir(&subdir2).unwrap();

    // Add files to subdirectories
    create_temp_file(&temp_dir, "subdir1/file3.txt", content1);
    create_temp_file(&temp_dir, "subdir2/file4.txt", content1);
    create_temp_file(&temp_dir, "subdir1/unique2.txt", b"Some other content");

    // First scan without cache
    let scanner = Scanner::new(false, None, None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();

    // Verify initial scan results
    assert_eq!(duplicates.len(), 1, "Should find one group of duplicates");
    for (_, files) in &duplicates {
        assert_eq!(files.len(), 4, "Should find 4 duplicate files");
        assert_eq!(files[0].size, 13, "Files should be 13 bytes");
    }

    // Modify one duplicate file
    let modified_file = temp_dir.path().join("file1.txt");
    let mut file = File::create(&modified_file).unwrap();
    file.write_all(b"Modified content").unwrap();

    // Scan again
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();

    // Verify modified results
    assert_eq!(
        duplicates.len(),
        1,
        "Should still find one group of duplicates"
    );
    for (_, files) in &duplicates {
        assert_eq!(files.len(), 3, "Should now find 3 duplicate files");
    }
}

#[test]
fn test_scan_with_cache() {
    let temp_dir = TempDir::new().unwrap();
    let content = b"Test content";

    // Create duplicate files
    create_temp_file(&temp_dir, "file1.txt", content);
    create_temp_file(&temp_dir, "file2.txt", content);

    // First scan with cache enabled
    let scanner1 = Scanner::new(true, None, None).unwrap();
    let duplicates1 = scanner1.find_duplicates(temp_dir.path()).unwrap();

    // Create a new scanner (simulating a new program run)
    let scanner2 = Scanner::new(true, None, None).unwrap();
    let duplicates2 = scanner2.find_duplicates(temp_dir.path()).unwrap();

    // Results should be identical
    assert_eq!(
        duplicates1.len(),
        duplicates2.len(),
        "Cache should provide consistent results"
    );

    // Verify that cache is being used
    let cache = Cache::new().unwrap();
    let file_path = temp_dir.path().join("file1.txt");
    assert!(
        cache
            .get_hash(&file_path, content.len() as u64)
            .unwrap()
            .is_some(),
        "Hash should be stored in cache"
    );
}

#[test]
fn test_scan_with_size_filters() {
    let temp_dir = TempDir::new().unwrap();

    // Create files of different sizes
    create_temp_file(&temp_dir, "small1.txt", b"small");
    create_temp_file(&temp_dir, "small2.txt", b"small");
    create_temp_file(&temp_dir, "medium1.txt", &vec![b'a'; 1000]);
    create_temp_file(&temp_dir, "medium2.txt", &vec![b'a'; 1000]);
    create_temp_file(&temp_dir, "large1.txt", &vec![b'b'; 10000]);
    create_temp_file(&temp_dir, "large2.txt", &vec![b'b'; 10000]);

    // Test minimum size filter
    let scanner = Scanner::new(false, Some(100), None).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    assert_eq!(
        duplicates.len(),
        2,
        "Should find medium and large duplicates"
    );

    // Test maximum size filter
    let scanner = Scanner::new(false, None, Some(5000)).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    assert_eq!(
        duplicates.len(),
        2,
        "Should find small and medium duplicates"
    );

    // Test both filters
    let scanner = Scanner::new(false, Some(100), Some(5000)).unwrap();
    let duplicates = scanner.find_duplicates(temp_dir.path()).unwrap();
    assert_eq!(duplicates.len(), 1, "Should find only medium duplicates");
}

#[test]
fn test_error_handling() {
    // Test with nonexistent directory
    let scanner = Scanner::new(false, None, None).unwrap();
    let result = scanner.find_duplicates(PathBuf::from("/nonexistent/path").as_path());
    assert!(
        result.is_err(),
        "Should handle nonexistent directory gracefully"
    );

    // Test with unreadable directory (platform-specific)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create a file
        create_temp_file(&temp_dir, "test.txt", b"test");

        // Remove read permissions
        let metadata = fs::metadata(path).unwrap();
        let mut perms = metadata.permissions();
        perms.set_mode(0o000);
        fs::set_permissions(path, perms).unwrap();

        let result = scanner.find_duplicates(path);
        assert!(
            result.is_err(),
            "Should handle unreadable directory gracefully"
        );
    }
}
