use anyhow::Result;
use directories::ProjectDirs;
use log::debug;
use rusqlite::{params, Connection, OpenFlags};
use std::{fs, path::Path, sync::Mutex};

pub struct Cache {
    conn: Mutex<Connection>,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "dupcheck", "DupCheck")
            .ok_or_else(|| anyhow::anyhow!("Could not determine project directories"))?;

        let cache_dir = project_dirs.cache_dir();
        fs::create_dir_all(cache_dir)?;

        let db_path = cache_dir.join("cache.db");
        debug!("Using cache database at: {}", db_path.display());

        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS file_hashes (
                path TEXT PRIMARY KEY,
                size INTEGER NOT NULL,
                hash TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Cache {
            conn: Mutex::new(conn),
        })
    }

    pub fn get_hash(&self, path: &Path, size: u64) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT hash FROM file_hashes WHERE path = ? AND size = ?")?;

        let path_str = path.to_string_lossy();
        let result = stmt.query_row(params![path_str.as_ref(), size], |row| {
            row.get::<_, String>(0)
        });

        match result {
            Ok(hash) => Ok(Some(hash)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn store_hash(&self, path: &Path, size: u64, hash: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let path_str = path.to_string_lossy();

        conn.execute(
            "INSERT OR REPLACE INTO file_hashes (path, size, hash) VALUES (?, ?, ?)",
            params![path_str.as_ref(), size, hash],
        )?;

        Ok(())
    }
}
