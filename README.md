# File Organizer

A CLI tool written in Rust that organizes files in a directory by sorting them into folders based on their file extension.

## Categories

| Folder     | Extensions                                              |
|------------|---------------------------------------------------------|
| Images     | jpg, jpeg, png, gif, bmp, svg, webp, ico               |
| Videos     | mp4, mkv, avi, mov, wmv, flv                            |
| Audio      | mp3, wav, flac, aac, ogg, wma                           |
| Documents  | pdf, doc, docx, xls, xlsx, ppt, pptx, txt, csv, rtf    |
| Archives   | zip, rar, 7z, tar, gz, bz2                              |
| Code       | rs, py, js, ts, c, cpp, h, java, go, rb, html, css     |
| Programs   | exe, msi, dmg, app, deb, rpm                            |
| Other      | Everything else                                         |

## Installation

```bash
git clone https://github.com/omar-dakalbab/file-organizer.git
cd file-organizer
cargo build --release
```

## Usage

```bash
# Organize files in a directory
cargo run -- ~/Downloads

# Dry run (preview without moving files)
cargo run -- ~/Downloads --dry-run

# Organize current directory
cargo run
```

## Features

- Sorts files into categorized folders by extension
- Dry run mode to preview changes before applying
- Handles filename conflicts automatically
- Skips directories (only moves files)
