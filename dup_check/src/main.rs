//! DupCheck - A Safe Duplicate File Finder
//! 
//! This utility helps users find and safely manage duplicate files in their system.
//! It provides features like:
//! - Scanning directories for duplicate files
//! - Filtering by file size with human-readable formats (e.g., 1K, 1M, 1G)
//! - Safe deletion of duplicates while preserving originals
//! - Progress tracking and detailed statistics
//! - Caching for improved performance
//! 
//! # Usage
//! ```bash
//! dupcheck [OPTIONS]
//! 
//! Options:
//!   -p, --path <PATH>      Directory to scan (default: current directory)
//!   -n, --min-size <SIZE>  Minimum file size (e.g., 1K, 1M)
//!   -x, --max-size <SIZE>  Maximum file size (e.g., 1G)
//!   -c, --no-cache        Disable hash caching
//! ```

use anyhow::{Result, Context};
use dup_check::{cli, scanner::Scanner, ui, interactive};
use env_logger;
use dialoguer::{theme::ColorfulTheme, Confirm};

fn main() -> Result<()> {
    env_logger::init();
    let theme = ColorfulTheme::default();
    
    // Get configuration either from CLI args or interactive mode
    let mut config = if std::env::args().len() > 1 {
        // Use CLI args if provided
        let args = cli::parse_args();
        interactive::InteractiveConfig {
            path: args.path,
            min_size: args.min_size,
            max_size: args.max_size,
            use_cache: !args.no_cache,
        }
    } else {
        // Use interactive mode if no args provided
        interactive::get_interactive_config()?
    };

    loop {
        let scanner = Scanner::new(config.use_cache, config.min_size, config.max_size)
            .context("Failed to initialize scanner")?;
        
        let duplicates = scanner.find_duplicates(config.path.as_path())
            .context("Failed to scan for duplicates")?;
        
        ui::display_duplicates(&duplicates);

        if !duplicates.is_empty() {
            println!("\nWould you like to delete duplicate files? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().eq_ignore_ascii_case("y") {
                ui::delete_duplicates(&duplicates)
                    .context("Failed to delete duplicates")?;
            }
        } else {
            println!("\nNo duplicates found!");
        }

        // Ask if user wants to scan another directory
        let scan_another = Confirm::with_theme(&theme)
            .with_prompt("\nWould you like to scan another directory?")
            .default(true)
            .interact()?;

        if !scan_another {
            break;
        }

        // Get new configuration for next scan
        config = interactive::get_interactive_config()?;
    }

    Ok(())
}
