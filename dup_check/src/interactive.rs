use std::path::PathBuf;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use console::Term;

pub struct InteractiveConfig {
    pub path: PathBuf,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub use_cache: bool,
}

fn parse_size(input: &str) -> Option<u64> {
    let input = input.trim().to_uppercase();
    let mut chars = input.chars().peekable();
    let mut number = String::new();
    let mut unit = String::new();

    // Parse the number part
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() || *c == '.' {
            number.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    // Parse the unit part
    while let Some(c) = chars.next() {
        if c.is_ascii_alphabetic() {
            unit.push(c);
        }
    }

    // Parse the number
    let value: f64 = number.parse().ok()?;

    // Convert to bytes based on unit
    let multiplier = match unit.as_str() {
        "" | "B" => 1_u64,
        "K" | "KB" => 1024_u64,
        "M" | "MB" => 1024_u64 * 1024,
        "G" | "GB" => 1024_u64 * 1024 * 1024,
        "T" | "TB" => 1024_u64 * 1024 * 1024 * 1024,
        _ => return None,
    };

    Some((value * multiplier as f64) as u64)
}

pub fn get_interactive_config() -> Result<InteractiveConfig> {
    let theme = ColorfulTheme::default();
    let term = Term::stdout();
    term.clear_screen()?;

    println!("🔍 DupCheck - Duplicate File Finder");
    println!("===================================\n");

    // Directory selection
    let path = select_directory(&theme)?;

    // Size filters
    let use_size_filters = Confirm::with_theme(&theme)
        .with_prompt("Would you like to set size filters?")
        .default(false)
        .interact()?;

    let (min_size, max_size) = if use_size_filters {
        // Minimum size
        let use_min = Confirm::with_theme(&theme)
            .with_prompt("Set minimum file size?")
            .default(false)
            .interact()?;
        
        let min_size = if use_min {
            let mut size_input;
            let mut parsed_size = None;
            
            println!("\nEnter minimum size (e.g., 1MB, 500KB, 1.5GB):");
            while parsed_size.is_none() {
                size_input = Input::<String>::with_theme(&theme)
                    .with_prompt("Size")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if parse_size(input).is_some() {
                            Ok(())
                        } else {
                            Err("Invalid size format. Examples: 1MB, 500KB, 1.5GB")
                        }
                    })
                    .interact()?;
                
                parsed_size = parse_size(&size_input);
            }
            parsed_size
        } else {
            None
        };

        // Maximum size
        let use_max = Confirm::with_theme(&theme)
            .with_prompt("Set maximum file size?")
            .default(false)
            .interact()?;
        
        let max_size = if use_max {
            let mut size_input;
            let mut parsed_size = None;
            
            println!("\nEnter maximum size (e.g., 1MB, 500KB, 1.5GB):");
            while parsed_size.is_none() {
                size_input = Input::<String>::with_theme(&theme)
                    .with_prompt("Size")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if parse_size(input).is_some() {
                            Ok(())
                        } else {
                            Err("Invalid size format. Examples: 1MB, 500KB, 1.5GB")
                        }
                    })
                    .interact()?;
                
                parsed_size = parse_size(&size_input);
            }
            parsed_size
        } else {
            None
        };

        (min_size, max_size)
    } else {
        (None, None)
    };

    // Cache option
    let use_cache = Confirm::with_theme(&theme)
        .with_prompt("Use cache for faster subsequent scans?")
        .default(true)
        .interact()?;

    // Configuration summary
    term.clear_screen()?;
    println!("📋 Scan Configuration:");
    println!("Directory: {}", path.display());
    if let Some(min) = min_size {
        println!("Minimum size: {}", format_size(min));
    }
    if let Some(max) = max_size {
        println!("Maximum size: {}", format_size(max));
    }
    println!("Cache enabled: {}", if use_cache { "Yes" } else { "No" });
    println!("\nPress Enter to start scanning...");
    term.read_line()?;

    Ok(InteractiveConfig {
        path,
        min_size,
        max_size,
        use_cache,
    })
}

fn select_directory(theme: &ColorfulTheme) -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    
    loop {
        let mut entries = vec![
            String::from("📂 Select current directory"),
            String::from("📁 Parent directory"),
            String::from("🏠 Go to home directory"),
        ];

        // Add subdirectories
        let mut subdirs: Vec<_> = std::fs::read_dir(&current_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.path())
            .collect();
        subdirs.sort();

        for subdir in &subdirs {
            entries.push(format!("   {}", subdir.file_name().unwrap().to_string_lossy()));
        }

        println!("\nCurrent directory: {}", current_dir.display());
        let selection = Select::with_theme(theme)
            .with_prompt("Choose directory")
            .items(&entries)
            .default(0)
            .interact()?;

        match selection {
            0 => return Ok(current_dir), // Select current directory
            1 => { // Parent directory
                if let Some(parent) = current_dir.parent() {
                    current_dir = parent.to_path_buf();
                }
            },
            2 => current_dir = home_dir.clone(), // Home directory
            n => { // Selected subdirectory
                let selected = &subdirs[n - 3];
                current_dir = selected.clone();
            }
        }
    }
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
