use crate::file_info::FileInfo;
use anyhow::Result;
use console::Term;
use std::collections::HashMap;

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

pub fn display_duplicates(duplicates: &HashMap<String, Vec<FileInfo>>) {
    if duplicates.is_empty() {
        println!("\n✨ No duplicates found!");
        return;
    }

    let term = Term::stdout();
    let _ = term.clear_screen();

    let total_groups = duplicates.len();
    let total_files: usize = duplicates.values().map(|files| files.len()).sum();
    let total_wasted: u64 = duplicates
        .values()
        .map(|files| files[0].size * (files.len() as u64 - 1))
        .sum();

    println!("\n📊 Duplicate Files Summary");
    println!("========================");
    println!("🔍 Found {} duplicate groups", total_groups);
    println!("📁 Total duplicate files: {}", total_files);
    println!("💾 Wasted space: {}\n", format_size(total_wasted));

    for (i, (_, files)) in duplicates.iter().enumerate() {
        let size = format_size(files[0].size);
        println!("Group {} (Size: {})", i + 1, size);
        println!("-------------------");
        
        for (j, file) in files.iter().enumerate() {
            let symbol = if j == 0 { "🔒" } else { "📄" };
            println!("{} {}", symbol, file.path.display());
        }
        println!();
    }

    println!("Legend:");
    println!("🔒 Original file (will be kept)");
    println!("📄 Duplicate file (can be deleted)");
}

pub fn delete_duplicates(duplicates: &HashMap<String, Vec<FileInfo>>) -> Result<()> {
    let mut total_deleted = 0;
    let mut space_freed = 0u64;

    for files in duplicates.values() {
        // Skip the first file (original)
        for file in files.iter().skip(1) {
            if std::fs::remove_file(&file.path).is_ok() {
                total_deleted += 1;
                space_freed += file.size;
                println!("✅ Deleted: {}", file.path.display());
            } else {
                println!("❌ Failed to delete: {}", file.path.display());
            }
        }
    }

    println!("\n🧹 Cleanup Summary");
    println!("================");
    println!("✨ Deleted {} duplicate files", total_deleted);
    println!("💾 Freed up {}", format_size(space_freed));

    Ok(())
}
