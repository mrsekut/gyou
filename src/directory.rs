use std::{
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct DirItem {
    pub path: PathBuf,
    pub count: usize,
    pub is_dir: bool,
}

pub fn list_dir_items(root: &Path, exts: &[String]) -> Result<Vec<DirItem>, io::Error> {
    let mut results = fs::read_dir(root)?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();

            if path.is_dir() {
                Some(DirItem {
                    path: path.clone(),
                    count: sum_lines_in_dir(&path, exts),
                    is_dir: true,
                })
            } else if path.is_file() && has_the_extention(&path, exts) {
                Some(DirItem {
                    path: path.clone(),
                    count: count_lines(&path),
                    is_dir: false,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    results.sort_by(|a, b| b.count.cmp(&a.count));
    Ok(results)
}

fn sum_lines_in_dir(dir: &Path, exts: &[String]) -> usize {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.file_type().is_file())
        .filter(|f| has_the_extention(f.path(), exts))
        .map(|f| count_lines(f.path()))
        .sum()
}

fn has_the_extention(path: &Path, exts: &[String]) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| exts.contains(&ext.to_string()))
        .unwrap_or(false)
}

fn count_lines(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|content| content.lines().count())
        .unwrap_or(0)
}
