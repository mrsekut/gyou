use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::ListState;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::directory::DirItem;

pub fn handle_event(
    current_dir: &str,
    base: &str,
    state: &mut ListState,
    items: &[DirItem],
) -> Option<String> {
    if let Event::Key(key) = event::read().ok()? {
        match key.code {
            KeyCode::Left => {
                if is_at_root(current_dir, base) {
                    return None;
                }
                return move_to_parent(current_dir, base);
            }
            KeyCode::Right => {
                if let Some(i) = state.selected() {
                    if items.get(i).map(|v| v.is_dir).unwrap_or(false) {
                        return Some(items[i].path.clone());
                    }
                }
                return None;
            }
            _ => {}
        }
    }
    None
}

fn is_at_root(current_dir: &str, base: &str) -> bool {
    let canon_current = canonicalize_path(current_dir);
    let canon_base = canonicalize_path(base);
    canon_current == canon_base
}

fn move_to_parent(current_dir: &str, base: &str) -> Option<String> {
    let parent = Path::new(current_dir).parent()?;
    let canon_parent = canonicalize_path(parent.to_str().unwrap_or(current_dir));
    let canon_base = canonicalize_path(base);

    if canon_parent.starts_with(&canon_base) {
        Some(parent.to_string_lossy().to_string())
    } else {
        None
    }
}

fn canonicalize_path(path: &str) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| Path::new(path).to_path_buf())
}
