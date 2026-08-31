use std::fs;

fn main() {
    let exts = ["jpg", "jpeg", "png", "webp", "gif", "webm", "mp4"];
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                if name == "failed.log" || exts.iter().any(|ext| name.ends_with(&format!(".{}", ext))) {
                    if fs::remove_file(&path).is_ok() {
                        println!("Deleted {}", path.display());
                        count += 1;
                    }
                }
            }
        }
    }
    println!("Cleaned {} media files.", count);
}
