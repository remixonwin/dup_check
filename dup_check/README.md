# Duplicate File Finder

A powerful Rust-based tool for finding and managing duplicate files on your system. This tool can scan directories for duplicate files, show detailed statistics, and optionally remove duplicates while keeping one copy.

## Features

- 🔍 Finds duplicate files using SHA-256 hash comparison
- 📁 Supports scanning specific file types or categories
- 📊 Provides detailed statistics about duplicates
- 🗑️ Optional duplicate deletion with safety confirmations
- 🎨 Interactive mode with user-friendly prompts
- 📈 Progress bars and real-time scanning status

## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Usage

### Interactive Mode (Recommended for New Users)

Simply run the program without arguments:
```bash
cargo run
```
This will guide you through:
1. Selecting a directory to scan
2. Choosing file types to look for
3. Viewing results and optionally deleting duplicates

### Command-Line Options

```bash
cargo run -- [OPTIONS]
```

Available options:

| Option | Description |
|--------|-------------|
| `-d, --dir <PATH>` | Directory to scan (optional, will prompt if not provided) |
| `-r, --delete` | Delete duplicate files (keeps one copy) |
| `-e, --extensions <LIST>` | Specific file extensions to scan (comma-separated) |
| `-y, --yes` | Skip deletion confirmation |
| `-l, --list-categories` | Show predefined file type categories |

### Common Usage Examples

1. **List Available File Categories:**
   ```bash
   cargo run -- --list-categories
   ```

2. **Scan a Specific Directory:**
   ```bash
   cargo run -- --dir "C:\Users\YourName\Documents"
   ```

3. **Scan for Specific File Types:**
   ```bash
   cargo run -- --dir "C:\Users\YourName\Pictures" --extensions "jpg,png,gif"
   ```

4. **Scan and Delete Duplicates (with confirmation):**
   ```bash
   cargo run -- --dir "C:\Users\YourName\Downloads" --delete
   ```

5. **Scan and Delete Duplicates (without confirmation):**
   ```bash
   cargo run -- --dir "C:\Users\YourName\Downloads" --delete --yes
   ```

## Supported File Categories

The tool includes predefined categories for common file types:

1. **Images**
   - Extensions: jpg, jpeg, png, gif, bmp, tiff, webp, svg, ico

2. **Videos**
   - Extensions: mp4, avi, mkv, mov, wmv, flv, webm, m4v, 3gp

3. **Documents**
   - Extensions: pdf, doc, docx, xls, xlsx, ppt, pptx, txt, rtf, odt, ods, odp

4. **Audio**
   - Extensions: mp3, wav, flac, m4a, aac, ogg, wma

5. **Archives**
   - Extensions: zip, rar, 7z, tar, gz, bz2, xz

6. **Code**
   - Extensions: rs, py, js, html, css, cpp, c, h, java, php, rb, go, ts

## Safety Features

- Minimum file size threshold to avoid system files
- Confirmation prompt before deletion
- Keeps one copy of each duplicate file
- Progress tracking and error reporting
- Handles read permission errors gracefully

## Statistics and Reporting

The tool provides detailed information about:
- Total number of duplicate files
- Total space that could be saved
- Duplicates grouped by file type
- Space wasted per file type
- Successful and failed deletions

## Error Handling

- Validates directory paths
- Reports inaccessible files
- Shows warning for failed deletions
- Provides clear error messages

## Performance Considerations

- Uses efficient file hashing
- Skips files smaller than 1KB
- Optimized progress updates
- Memory-efficient file scanning

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
