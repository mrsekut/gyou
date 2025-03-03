use std::fs;
use std::io;
use walkdir::WalkDir;

pub fn list_dir_items(root: &str, exts: &[&str]) -> io::Result<Vec<(String, usize, bool)>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(root)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let sum: usize = WalkDir::new(&path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|f| f.file_type().is_file())
                    .filter_map(|f| {
                        f.path()
                            .extension()
                            .and_then(|s| s.to_str())
                            .filter(|ext| exts.contains(ext))
                            .map(|_| fs::read_to_string(f.path()).ok())
                    })
                    .flatten()
                    .map(|content| content.lines().count())
                    .sum();
                results.push((path.to_string_lossy().to_string(), sum, true));
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if exts.contains(&ext) {
                        let count = fs::read_to_string(&path)
                            .map(|content| content.lines().count())
                            .unwrap_or(0);
                        results.push((path.to_string_lossy().to_string(), count, false));
                    }
                }
            }
        }
    }
    results.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(results)
}
