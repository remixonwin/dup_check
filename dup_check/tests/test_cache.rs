use dup_check::cache::Cache;
use std::fs::File;
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
fn test_cache_store_and_retrieve() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Hello, World!");
    
    let cache = Cache::new().unwrap();
    let hash = "test_hash".to_string();
    let size = 13;
    
    // Store hash
    cache.store_hash(&file_path, size, &hash).unwrap();
    
    // Retrieve hash
    let retrieved = cache.get_hash(&file_path, size).unwrap();
    assert_eq!(retrieved, Some(hash), "Retrieved hash should match stored hash");
}

#[test]
fn test_cache_size_mismatch() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Hello, World!");
    
    let cache = Cache::new().unwrap();
    let hash = "test_hash".to_string();
    
    // Store hash with original size
    cache.store_hash(&file_path, 13, &hash).unwrap();
    
    // Try to retrieve with different size
    let retrieved = cache.get_hash(&file_path, 14).unwrap();
    assert_eq!(retrieved, None, "Should not retrieve hash when size doesn't match");
}

#[test]
fn test_cache_nonexistent_file() {
    let cache = Cache::new().unwrap();
    let nonexistent = PathBuf::from("/nonexistent/file.txt");
    
    let retrieved = cache.get_hash(&nonexistent, 0).unwrap();
    assert_eq!(retrieved, None, "Should return None for nonexistent file");
}

#[test]
fn test_cache_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let cache = Cache::new().unwrap();
    
    // Create multiple files with different content
    let files = vec![
        (create_temp_file(&temp_dir, "file1.txt", b"Content 1"), "hash1", 9),
        (create_temp_file(&temp_dir, "file2.txt", b"Different"), "hash2", 9),
        (create_temp_file(&temp_dir, "file3.txt", b"Test data"), "hash3", 9),
    ];
    
    // Store all hashes
    for (path, hash, size) in &files {
        cache.store_hash(path, *size, hash).unwrap();
    }
    
    // Verify all hashes
    for (path, expected_hash, size) in &files {
        let retrieved = cache.get_hash(path, *size).unwrap();
        assert_eq!(retrieved, Some(expected_hash.to_string()),
                  "Retrieved hash should match stored hash for each file");
    }
}

#[test]
fn test_cache_update_hash() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_file(&temp_dir, "test.txt", b"Initial content");
    let size = 15;
    
    let cache = Cache::new().unwrap();
    
    // Store initial hash
    cache.store_hash(&file_path, size, "initial_hash").unwrap();
    
    // Update with new hash
    cache.store_hash(&file_path, size, "updated_hash").unwrap();
    
    // Verify updated hash
    let retrieved = cache.get_hash(&file_path, size).unwrap();
    assert_eq!(retrieved, Some("updated_hash".to_string()),
              "Retrieved hash should match updated hash");
}

#[test]
fn test_cache_concurrent_access() {
    use std::sync::Arc;
    use std::thread;
    
    let temp_dir = TempDir::new().unwrap();
    let cache = Arc::new(Cache::new().unwrap());
    let mut handles = vec![];
    
    // Create 10 threads that simultaneously access the cache
    for i in 0..10 {
        let cache = Arc::clone(&cache);
        let temp_dir = temp_dir.path().to_path_buf();
        
        let handle = thread::spawn(move || {
            let file_name = format!("file{}.txt", i);
            let file_path = temp_dir.join(file_name);
            let content = format!("Content {}", i);
            
            // Create file
            let mut file = File::create(&file_path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            
            // Store and retrieve hash
            let hash = format!("hash{}", i);
            cache.store_hash(&file_path, content.len() as u64, &hash).unwrap();
            
            let retrieved = cache.get_hash(&file_path, content.len() as u64).unwrap();
            assert_eq!(retrieved, Some(hash), "Retrieved hash should match in thread {}", i);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
