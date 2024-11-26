//! DupCheck - A Safe Duplicate File Finder
//! 
//! This utility helps users find and safely manage duplicate files in their system.
//! It provides features like:
//! - Scanning directories for duplicate files
//! - Filtering by file size
//! - Safe deletion of duplicates while preserving originals
//! - Progress tracking and detailed statistics
//! - Caching for improved performance
//! 
//! # Usage
//! ```bash
//! dupcheck --path <directory> [OPTIONS]
//! ```

use anyhow::Result;
use dup_check::{cli, scanner::Scanner, ui};
use env_logger;

fn main() -> Result<()> {
    env_logger::init();
    let args = cli::parse_args();

    let scanner = Scanner::new(!args.no_cache, args.min_size, args.max_size)?;
    let duplicates = scanner.find_duplicates(&args.path)?;
    
    ui::display_duplicates(&duplicates);

    if !duplicates.is_empty() {
        println!("\nWould you like to delete duplicate files? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            ui::delete_duplicates(&duplicates)?;
        }
    }

    Ok(())
}
