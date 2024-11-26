use crate::{cache::Cache, file_info::FileInfo, utils};
use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::debug;
use rayon::prelude::*;
use std::{collections::HashMap, path::Path, sync::Arc};
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

    pub fn find_duplicates(&self, path: &Path) -> Result<HashMap<String, Vec<FileInfo>>> {
        // Check if the directory exists first
        if !path.exists() {
            return Err(anyhow::anyhow!(
                "Directory does not exist: {}",
                path.display()
            ));
        }

        let multi_progress = MultiProgress::new();

        // File scanning progress
        let scan_progress = multi_progress.add(ProgressBar::new_spinner());
        scan_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        scan_progress.set_message("Collecting files...");

        // First pass: count total files for progress bar
        let total_files: u64 = WalkDir::new(path)
            .into_iter()
            .try_fold(0, |acc, entry| {
                let entry = entry?;
                Ok::<_, anyhow::Error>(if self.should_process_file(&entry) {
                    acc + 1
                } else {
                    acc
                })
            })?;

        scan_progress.set_message(format!("Found {} files to process", total_files));

        // Second pass: collect files into size groups with progress
        let mut size_groups: HashMap<u64, Vec<FileInfo>> = HashMap::new();
        let mut processed = 0;
        for entry_result in WalkDir::new(path).into_iter() {
            let entry = entry_result?;
            if !self.should_process_file(&entry) {
                continue;
            }

            let metadata = entry.metadata()?;
            let size = metadata.len();
            size_groups
                .entry(size)
                .or_default()
                .push(FileInfo::new(entry.path().to_path_buf(), size));

            processed += 1;
            if processed % 100 == 0 || processed == total_files {
                scan_progress.set_message(format!("Processed {}/{} files", processed, total_files));
            }
        }

        scan_progress.finish_with_message(format!("Processed {} files", total_files));

        // Hash calculation progress
        let hash_progress = multi_progress.add(ProgressBar::new_spinner());
        hash_progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        hash_progress.set_message("Analyzing potential duplicates...");

        // Count potential duplicates for progress
        let potential_duplicates: usize = size_groups
            .values()
            .filter(|files| files.len() > 1)
            .map(|files| files.len())
            .sum();

        hash_progress.set_message(format!(
            "Found {} files in duplicate size groups",
            potential_duplicates
        ));

        // Process files in parallel and collect duplicates
        let duplicates: HashMap<String, Vec<FileInfo>> = size_groups
            .into_par_iter()
            .filter(|(_, files)| files.len() > 1)
            .map(|(_, files)| {
                let mut hash_groups: HashMap<String, Vec<FileInfo>> = HashMap::new();
                for file in files {
                    if let Ok(hash) =
                        Self::calculate_hash_cached(&file.path, file.size, self.cache.as_ref())
                    {
                        hash_groups.entry(hash).or_default().push(file);
                    }
                }
                hash_groups
                    .into_iter()
                    .filter(|(_, group)| group.len() > 1)
                    .collect::<Vec<_>>()
            })
            .inspect(|groups| {
                if !groups.is_empty() {
                    hash_progress.set_message(format!("Found {} duplicate groups", groups.len()));
                }
            })
            .flatten()
            .collect();

        hash_progress.finish_with_message(format!("Found {} duplicate groups", duplicates.len()));

        Ok(duplicates)
    }

    fn should_process_file(&self, entry: &walkdir::DirEntry) -> bool {
        if !entry.file_type().is_file() {
            return false;
        }

        if utils::is_hidden(entry.path()) {
            return false;
        }

        if let Ok(metadata) = entry.metadata() {
            let size = metadata.len();
            if let Some(min_size) = self.min_size {
                if size < min_size {
                    return false;
                }
            }
            if let Some(max_size) = self.max_size {
                if size > max_size {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn calculate_hash_cached(path: &Path, size: u64, cache: Option<&Arc<Cache>>) -> Result<String> {
        if let Some(cache) = cache {
            if let Some(hash) = cache.get_hash(path, size)? {
                debug!("Cache hit for {}", path.display());
                return Ok(hash);
            }
        }

        let hash = utils::calculate_hash(path)?;

        if let Some(cache) = cache {
            cache.store_hash(path, size, &hash)?;
        }

        Ok(hash)
    }
}
