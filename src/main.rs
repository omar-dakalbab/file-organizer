use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "file-organizer", about = "Organize files in a directory by extension")]
struct Args {
    /// Directory to organize
    #[arg(default_value = ".")]
    directory: String,

    /// Dry run — show what would happen without moving files
    #[arg(short, long)]
    dry_run: bool,
}

fn extension_to_folder(ext: &str) -> &str {
    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" => "Images",
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" => "Videos",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" => "Audio",
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "csv" | "rtf" => "Documents",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => "Archives",
        "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "java" | "go" | "rb" | "html" | "css" => "Code",
        "exe" | "msi" | "dmg" | "app" | "deb" | "rpm" => "Programs",
        _ => "Other",
    }
}

fn main() {
    let args = Args::parse();
    let dir = Path::new(&args.directory);

    if !dir.is_dir() {
        eprintln!("Error: '{}' is not a valid directory", args.directory);
        std::process::exit(1);
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            std::process::exit(1);
        }
    };

    let mut moved: HashMap<String, Vec<String>> = HashMap::new();
    let mut count = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("none");

        let folder_name = extension_to_folder(ext);
        let dest_dir = dir.join(folder_name);
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let dest_path = dest_dir.join(&file_name);

        if args.dry_run {
            println!("[DRY RUN] {} -> {}/", file_name, folder_name);
        } else {
            if !dest_dir.exists() {
                fs::create_dir_all(&dest_dir).unwrap();
            }
            // Handle name conflicts
            let final_path = unique_path(&dest_path);
            if let Err(e) = fs::rename(&path, &final_path) {
                eprintln!("Failed to move '{}': {}", file_name, e);
                continue;
            }
            println!("Moved: {} -> {}/", file_name, folder_name);
        }

        moved
            .entry(folder_name.to_string())
            .or_default()
            .push(file_name);
        count += 1;
    }

    println!("\n--- Summary ---");
    if count == 0 {
        println!("No files to organize.");
    } else {
        for (folder, files) in &moved {
            println!("{}: {} file(s)", folder, files.len());
        }
        println!("Total: {} file(s) organized", count);
    }
}

fn unique_path(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }
    let stem = path.file_stem().unwrap().to_string_lossy();
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let parent = path.parent().unwrap();
    let mut i = 1;
    loop {
        let new_name = format!("{}_{}{}", stem, i, ext);
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        i += 1;
    }
}
