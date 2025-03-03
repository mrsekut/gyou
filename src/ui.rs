use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    CompletedFrame, Terminal,
};
use std::{fs, io, path::Path};

use crate::directory::DirItem;

pub fn draw_ui<'a>(
    terminal: &'a mut Terminal<CrosstermBackend<io::Stdout>>,
    current_dir: &'a str,
    state: &'a mut ListState,
    list_items: &'a [ListItem<'a>],
) -> Result<CompletedFrame<'a>, io::Error> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(f.size());
        let title = format!("Code Volume - {}", current_dir);
        let list_block = Block::default().borders(Borders::ALL).title(title);
        let list = List::new(list_items.iter().cloned().collect::<Vec<_>>())
            .block(list_block)
            .highlight_symbol(">> ");
        let _ = f.render_stateful_widget(list, chunks[0], state);
        let footer_text = "<-: Parent   ->: Child   ↑↓: Select   q: Quit";
        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL).title("Instructions"));
        f.render_widget(footer, chunks[1]);
    })
}

pub fn handle_event(
    current_dir: &str,
    base: &str,
    state: &mut ListState,
    items: &[DirItem],
) -> Option<String> {
    if let Event::Key(key) = event::read().ok()? {
        match key.code {
            KeyCode::Char('q') => return Some("__exit__".to_string()),
            KeyCode::Left => {
                let canon_current = fs::canonicalize(current_dir)
                    .unwrap_or_else(|_| Path::new(current_dir).to_path_buf());
                let canon_base =
                    fs::canonicalize(base).unwrap_or_else(|_| Path::new(base).to_path_buf());
                if canon_current == canon_base {
                    return None;
                }
                if let Some(parent) = Path::new(current_dir).parent() {
                    let canon_parent =
                        fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
                    if canon_parent.starts_with(&canon_base) {
                        return Some(parent.to_string_lossy().to_string());
                    } else {
                        return None;
                    }
                } else {
                    return Some(current_dir.to_string());
                }
            }
            KeyCode::Right => {
                if let Some(i) = state.selected() {
                    if items.get(i).map(|v| v.is_dir).unwrap_or(false) {
                        return Some(items[i].path.clone());
                    }
                }
            }
            KeyCode::Up => {
                let idx = state.selected().unwrap_or(0);
                state.select(Some(if idx > 0 { idx - 1 } else { 0 }));
            }
            KeyCode::Down => {
                let idx = state.selected().unwrap_or(0);
                state.select(Some(if idx < items.len().saturating_sub(1) {
                    idx + 1
                } else {
                    idx
                }));
            }
            _ => {}
        }
    }
    None
}
