use crate::{cache::Cache, file_info::FileInfo, utils};
use anyhow::Result;
use indicatif::ProgressBar;
use log::debug;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::Arc,
    time::SystemTime,
};
use walkdir::WalkDir;

pub struct Scanner {
    cache: Option<Arc<Cache>>,
    min_size: Option<u64>,
    max_size: Option<u64>,
}

impl Scanner {
    pub fn new(use_cache: bool, min_size: Option<u64>, max_size: Option<u64>) -> Result<Self> {
        let cache = if use_cache {
            Some(Arc::new(Cache::new()?))
        } else {
            None
        };

        Ok(Scanner {
            cache,
            min_size,
            max_size,
        })
    }

    pub fn find_duplicates(&self, path: &str) -> Result<HashMap<String, Vec<FileInfo>>> {
        let progress = ProgressBar::new_spinner();
        progress.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        progress.set_message("Scanning files...");

        // First, collect all files into size groups
        let mut size_groups: HashMap<u64, Vec<FileInfo>> = HashMap::new();
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| self.should_process_file(e))
        {
            let size = entry.metadata()?.len();
            size_groups
                .entry(size)
                .or_default()
                .push(FileInfo::new(entry.path().to_owned(), size));
        }

        progress.set_message("Calculating hashes...");
        
        // Process files in parallel and collect results
        let cache = self.cache.clone();
        let duplicates: HashMap<String, Vec<FileInfo>> = size_groups
            .into_iter()
            .filter(|(_, files)| files.len() > 1)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(move |(_, files)| {
                let mut hash_groups: HashMap<String, Vec<FileInfo>> = HashMap::new();
                for file in files {
                    if let Ok(hash) = Self::calculate_hash_cached(&file.path, file.size, cache.as_ref()) {
                        let mut file = file.clone();
                        file.hash = Some(hash.clone());
                        hash_groups.entry(hash).or_default().push(file);
                    }
                }
                hash_groups.into_iter()
                    .filter(|(_, files)| files.len() > 1)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        progress.finish_with_message("Scan complete");
        Ok(duplicates)
    }

    fn should_process_file(&self, entry: &walkdir::DirEntry) -> bool {
        if entry.file_type().is_dir() || utils::is_hidden(entry.path()) {
            return false;
        }

        match entry.metadata() {
            Ok(metadata) => {
                let size = metadata.len();
                self.min_size.map_or(true, |min| size >= min)
                    && self.max_size.map_or(true, |max| size <= max)
            }
            Err(_) => false,
        }
    }

    fn calculate_hash_cached(path: &Path, size: u64, cache: Option<&Arc<Cache>>) -> Result<String> {
        if let Some(cache) = cache {
            let modified = fs::metadata(path)?
                .modified()?
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs();

            if let Some(cached_hash) = cache.get_hash(path, size, modified)? {
                debug!("Cache hit for {}", path.display());
                return Ok(cached_hash);
            }

            let hash = utils::calculate_hash(path)?;
            cache.insert_hash(path, size, modified, &hash)?;
            Ok(hash)
        } else {
            utils::calculate_hash(path)
        }
    }
}
