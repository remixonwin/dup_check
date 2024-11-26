use crate::{file_info::FileInfo, utils::format_size};
use anyhow::Result;
use std::{collections::HashMap, fs, io::{self, Write}};

pub fn display_duplicates(duplicates: &HashMap<String, Vec<FileInfo>>) {
    if duplicates.is_empty() {
        println!("No duplicates found.");
        return;
    }

    let total_groups = duplicates.len();
    let total_files: usize = duplicates.values().map(|files| files.len()).sum();
    let wasted_space: u64 = duplicates
        .values()
        .map(|files| files[0].size * (files.len() as u64 - 1))
        .sum();

    println!("\nDuplicate files found:");
    println!("----------------------");
    println!("Total groups: {}", total_groups);
    println!("Total duplicate files: {}", total_files);
    println!("Wasted space: {}", format_size(wasted_space));
    println!();

    for (hash, files) in duplicates.iter() {
        println!("Hash: {}", hash);
        println!("Size: {}", format_size(files[0].size));
        for (i, file) in files.iter().enumerate() {
            println!("{}: {}", i + 1, file.path.display());
        }
        println!();
    }
}

pub fn delete_duplicates(duplicates: &HashMap<String, Vec<FileInfo>>) -> Result<()> {
    for (hash, files) in duplicates {
        println!("\nDuplicate group (hash: {}):", hash);
        for (i, file) in files.iter().enumerate() {
            println!("{}: {}", i + 1, file.path.display());
        }

        println!("\nEnter the number of the file to keep (1-{}), or 's' to skip: ", files.len());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.eq_ignore_ascii_case("s") {
            continue;
        }

        if let Ok(keep_index) = input.parse::<usize>() {
            if keep_index >= 1 && keep_index <= files.len() {
                for (i, file) in files.iter().enumerate() {
                    if i != keep_index - 1 {
                        match fs::remove_file(&file.path) {
                            Ok(_) => println!("Deleted: {}", file.path.display()),
                            Err(e) => eprintln!("Failed to delete {}: {}", file.path.display(), e),
                        }
                    }
                }
            } else {
                println!("Invalid input. Skipping this group.");
            }
        } else {
            println!("Invalid input. Skipping this group.");
        }
    }
    Ok(())
}
