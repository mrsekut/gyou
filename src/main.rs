use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::collections::HashMap;
use std::env;
use std::io::{self, stdout};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    // ターミナルのセットアップ
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let args: Vec<String> = env::args().collect();
    let root = if args.len() > 1 { &args[1] } else { "src" };

    let counts = count_files_by_directory(root);

    let items: Vec<ListItem> = counts
        .iter()
        .map(|(path, count)| ListItem::new(format!("{:<40} {}", path, count)))
        .collect();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().borders(Borders::ALL).title("Code Volume");
            let list = List::new(items.clone()).block(block);
            f.render_widget(list, size);
        })?;

        if let Ok(true) = event::poll(std::time::Duration::from_millis(500)) {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break, // ESCキーで終了
                    _ => {}
                }
            }
        }
    }

    // ターミナルのリセット
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn count_files_by_directory(root: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let path = entry.path().parent().unwrap().to_string_lossy().to_string();
            *counts.entry(path).or_insert(0) += 1;
        }
    }

    counts
}
