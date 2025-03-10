<p align="center">English | <a href="README.zh-CN.md">中文</a></p>

# Clean Maven Failed Artifacts

A simple and practical Rust command-line tool for cleaning up failed downloads and corrupted packages in Maven repositories.

## Project Overview

When using Maven to build Java projects, network issues or other problems may cause dependency downloads to fail. These failed downloads remain in the local Maven repository as files ending with `.lastUpdated`. These files can cause subsequent build processes to repeatedly attempt and fail downloads, impacting development efficiency.

This tool scans the local Maven repository, automatically identifies and deletes all dependency directories containing `.lastUpdated` files, thereby resolving build failures.

## Features

- Automatically scans Maven repository directory
- Finds all files containing `.lastUpdated`
- Deletes folders with failed downloads
- Displays real-time scanning progress
- Shows final statistics of cleaned packages

## Usage

1. Run the program
2. Enter Maven repository path (e.g. `C:/Users/username/.m2/repository`)
3. The program will automatically scan and clean failed downloads

## Building

Ensure Rust environment is installed, then execute:

```bash
cargo build --release
```

The compiled executable will be generated in the `target/release` directory.

## Dependencies

- [walkdir](https://crates.io/crates/walkdir) v2.5.0 - For recursive directory traversal

## Important Notes

- Recommended to back up your Maven repository before use
- This tool permanently deletes files - use with caution

## License

MIT License © 2025 [Clover You](https://github.com/clovu)
