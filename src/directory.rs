use std::{fs, io, path::Path};
use walkdir::WalkDir;

pub struct DirItem {
    pub path: String,
    pub count: usize,
    pub is_dir: bool,
}

/// 指定したルートディレクトリ内の各エントリを処理し、
/// DirItem の Vec として返す。行数で降順ソートする。
pub fn list_dir_items(root: &str, exts: &[&str]) -> io::Result<Vec<DirItem>> {
    let mut results = Vec::new();

    for entry in fs::read_dir(root)? {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_dir() {
                results.push(process_directory(&path, exts));
            }
            if path.is_file() {
                if let Some(item) = process_file(&path, exts) {
                    results.push(item);
                }
            }
        }
    }
    results.sort_by(|a, b| b.count.cmp(&a.count));
    Ok(results)
}

fn process_directory(dir: &Path, exts: &[&str]) -> DirItem {
    let count = sum_lines_in_directory(dir, exts);
    DirItem {
        path: dir.to_string_lossy().into(),
        count,
        is_dir: true,
    }
}

/// 再帰的に対象拡張子のファイルの行数を合計する
fn sum_lines_in_directory(dir: &Path, exts: &[&str]) -> usize {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.file_type().is_file())
        .filter_map(|f| {
            f.path()
                .extension()
                .and_then(|s| s.to_str())
                .filter(|ext| exts.contains(ext))
                .and_then(|_| fs::read_to_string(f.path()).ok())
        })
        .map(|content| content.lines().count())
        .sum()
}

/// 対象拡張子なら DirItem を返す
fn process_file(file: &Path, exts: &[&str]) -> Option<DirItem> {
    if let Some(ext) = file.extension().and_then(|s| s.to_str()) {
        if exts.contains(&ext) {
            let count = fs::read_to_string(file)
                .map(|content| content.lines().count())
                .unwrap_or(0);

            return Some(DirItem {
                path: file.to_string_lossy().into(),
                count,
                is_dir: false,
            });
        }
    }
    None
}
