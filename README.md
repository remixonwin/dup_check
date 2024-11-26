# DupCheck - Safe Duplicate File Finder

DupCheck is a powerful and safe utility for finding and managing duplicate files on your system. It provides an interactive interface to scan directories, identify duplicates, and safely remove them while preserving original files.

## Features

- 🔍 Smart file scanning with SHA-256 hash comparison
- 💾 SQLite-based caching for faster subsequent scans
- ⚡ Parallel processing for improved performance
- 🗂️ Filter by file types or categories
- 🔒 Safe deletion with Windows API integration
- 📊 Progress tracking and detailed statistics
- 🎯 Interactive menu-driven interface
- 🔄 Thread-safe operations for reliable performance

## Installation

1. Ensure you have Rust installed on your system
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/dup_check.git
   cd dup_check
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The executable will be available at `target/release/dup_check`

## Usage

### Command Line Options

```bash
dup_check [OPTIONS]

Options:
  -d, --directory <PATH>     Directory to scan for duplicates
  -e, --extensions <LIST>    Comma-separated list of file extensions (e.g., "jpg,png,pdf")
  -y, --yes                  Skip confirmation prompts
  -h, --help                Display help information
  -V, --version             Display version information
```

### Interactive Menu

1. **Scan New Directory**
   - Enter the path to scan
   - Choose file types to include
   - View scanning progress
   - Benefit from cached results for faster rescans

2. **Change File Types**
   - Select from predefined categories:
     - Images (jpg, jpeg, png, gif, etc.)
     - Documents (pdf, doc, docx, txt, etc.)
     - Videos (mp4, avi, mkv, etc.)
     - Audio (mp3, wav, flac, etc.)
     - Archives (zip, rar, 7z, etc.)
     - Code (rs, py, js, etc.)
   - Or choose "All Files" to scan everything

3. **Show Current Results**
   - View duplicate groups
   - See file sizes and paths
   - Get total space usage statistics

4. **Delete Duplicates (Current File Types)**
   - Review duplicates before deletion
   - Confirm deletion action
   - See progress and results
   - Original files are preserved

5. **Delete ALL Duplicates**
   - Scan and delete duplicates of all file types
   - Extra confirmation required
   - Progress tracking during deletion

6. **Exit**
   - Safely exit the program

### Example Workflow

1. Start the program:
   ```bash
   dup_check
   ```

2. Choose option 1 to scan a new directory
   ```
   Please enter the directory path to scan:
   > C:\Users\YourName\Documents
   ```

3. Select file types to scan:
   ```
   Available file type categories:
   0. All Files (no extension filter)
   1. Images (jpg, jpeg, png, gif, ...)
   2. Documents (pdf, doc, docx, ...)
   ...
   ```

4. Review the results:
   ```
   Duplicate files found:
   Group 1 (hash: abc123...)
     Original: file1.jpg (1.2 MB)
     Duplicate 1: file2.jpg (1.2 MB)
     ...
   ```

5. Choose to delete duplicates if desired:
   ```
   WARNING: You are about to delete 5 duplicate files, saving 6.5 MB of space.
   This action cannot be undone!
   Are you sure you want to proceed? [y/N]:
   ```

## Performance Features

### Caching System
- SQLite-based file hash caching
- Persistent across program runs
- Automatic cache updates
- Thread-safe database access

### Parallel Processing
- Multi-threaded file scanning
- Parallel hash calculation
- Efficient memory usage
- Optimized for modern CPUs

## Safety Features

- Original files are always preserved
- Confirmation required before deletion
- Safe file handling using Windows API
- Progress tracking and error reporting
- Minimum file size threshold (1KB)
- Maximum path length checking
- Thread-safe operations

## File Type Categories

1. **Images**
   - jpg, jpeg, png, gif, bmp, tiff, webp

2. **Documents**
   - pdf, doc, docx, txt, rtf, odt, xlsx, xls, ppt, pptx

3. **Videos**
   - mp4, avi, mkv, mov, wmv, flv, webm

4. **Audio**
   - mp3, wav, flac, m4a, aac, ogg, wma

5. **Archives**
   - zip, rar, 7z, tar, gz, bz2, xz

6. **Code**
   - rs, py, js, html, css, cpp, c, h, java, php, rb, go, ts

## Tips

1. **Large Directories**
   - Initial scan caches results for faster subsequent scans
   - Parallel processing improves performance
   - Progress bar shows estimated time
   - Consider filtering by file type

2. **File Selection**
   - Use "All Files" option carefully
   - Consider scanning by category first
   - Review results before deletion

3. **Safe Usage**
   - Always verify duplicate groups
   - Keep backups of important files
   - Use the -y flag carefully

## Troubleshooting

1. **Long Paths**
   - Windows path length is limited to 260 characters
   - Move files closer to root if needed

2. **Access Denied**
   - Run as administrator for system folders
   - Check file permissions
   - Close programs using the files

3. **Performance**
   - First scan may be slower due to caching
   - Subsequent scans are significantly faster
   - Consider filtering by file type
   - Use SSD for better performance

4. **Cache Issues**
   - Cache is automatically maintained
   - Located in user's application data directory
   - Cleared when files are modified

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
