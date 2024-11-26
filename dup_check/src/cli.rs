use clap::Parser;
use std::path::PathBuf;

/// DupCheck - A safe and efficient duplicate file finder
#[derive(Parser, Debug)]
#[command(
    author = "DupCheck Team",
    version,
    about,
    long_about = "A safe and efficient duplicate file finder that helps you identify and manage duplicate files in your system. \
    It uses SHA-256 hashing and provides features like size filtering, caching, and safe deletion."
)]
pub struct Args {
    /// Directory path to scan for duplicates (defaults to current directory if not specified)
    #[arg(short = 'p', long, default_value = ".")]
    pub path: PathBuf,

    /// Minimum file size to consider (e.g., '1K' for 1 kilobyte, '1M' for 1 megabyte)
    #[arg(short = 'n', long, value_parser = parse_size)]
    pub min_size: Option<u64>,

    /// Maximum file size to consider (e.g., '1G' for 1 gigabyte)
    #[arg(short = 'x', long, value_parser = parse_size)]
    pub max_size: Option<u64>,

    /// Disable caching of file hashes (caching is enabled by default)
    #[arg(short = 'c', long)]
    pub no_cache: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}

/// Parse human-readable sizes like "1K", "1M", "1G"
fn parse_size(size_str: &str) -> Result<u64, String> {
    let size_str = size_str.trim().to_uppercase();
    let len = size_str.len();
    if len == 0 {
        return Err("Size cannot be empty".to_string());
    }

    let (num_str, suffix) = if let Some(last_char) = size_str.chars().last() {
        match last_char {
            'K' | 'M' | 'G' | 'T' => (&size_str[..len - 1], last_char),
            '0'..='9' => (size_str.as_str(), 'B'),
            _ => return Err(format!("Invalid size suffix: {}", last_char)),
        }
    } else {
        return Err("Invalid size format".to_string());
    };

    let base_size = num_str
        .parse::<u64>()
        .map_err(|_| format!("Invalid number: {}", num_str))?;

    let multiplier = match suffix {
        'K' => 1024,
        'M' => 1024 * 1024,
        'G' => 1024 * 1024 * 1024,
        'T' => 1024 * 1024 * 1024 * 1024,
        'B' => 1,
        _ => return Err(format!("Invalid size suffix: {}", suffix)),
    };

    Ok(base_size * multiplier)
}
