use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::{fs, path::Path, sync::{Arc, Mutex}};

pub struct Cache {
    conn: Arc<Mutex<Connection>>,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "dupcheck", "DupCheck")
            .context("Failed to get project directories")?;
        let cache_dir = project_dirs.cache_dir();
        fs::create_dir_all(cache_dir).context("Failed to create cache directory")?;
        let db_path = cache_dir.join("cache.db");

        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS file_cache (
                path TEXT PRIMARY KEY,
                size INTEGER NOT NULL,
                modified INTEGER NOT NULL,
                hash TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Cache {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn get_hash(&self, path: &Path, size: u64, modified: u64) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT hash FROM file_cache WHERE path = ? AND size = ? AND modified = ?",
            params![path.to_string_lossy(), size, modified],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(hash) => Ok(Some(hash)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn insert_hash(&self, path: &Path, size: u64, modified: u64, hash: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO file_cache (path, size, modified, hash) VALUES (?, ?, ?, ?)",
            params![path.to_string_lossy(), size, modified, hash],
        )?;
        Ok(())
    }

    pub fn clean_old_entries(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM file_cache WHERE path NOT IN (SELECT path FROM file_cache WHERE 1=0)",
            [],
        )?;
        Ok(())
    }
}
