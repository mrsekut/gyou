use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::{self, stdout};
use std::path::Path;
use walkdir::WalkDir;

// 新規: 指定拡張子(ts, tsx)のファイルについて、直下のみのサブディレクトリごとに行数合計を計算
fn count_lines_by_extension(root: &str, exts: &[&str]) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    // ルート直下のエントリのみ取得
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                // サブディレクトリ内の対象ファイルを再帰的に探索
                let mut sum = 0;
                for file in walkdir::WalkDir::new(&path)
                    .into_iter()
                    .filter_map(Result::ok)
                {
                    if file.file_type().is_file() {
                        if let Some(ext) = file.path().extension().and_then(|s| s.to_str()) {
                            if exts.contains(&ext) {
                                if let Ok(content) = fs::read_to_string(file.path()) {
                                    sum += content.lines().count();
                                }
                            }
                        }
                    }
                }
                counts.insert(path.to_string_lossy().to_string(), sum);
            }
        }
    }
    counts
}

fn main() -> io::Result<()> {
    // ターミナルのセットアップ
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let args: Vec<String> = env::args().collect();
    let root = if args.len() > 1 { &args[1] } else { "src" };
    let mut current_dir = root.to_string();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        // 対象拡張子(ts, tsx)で行数合計を計算し、降順ソート
        let mut counts = count_lines_by_extension(&current_dir, &["ts", "tsx"]);
        let mut items: Vec<(String, usize)> = counts.drain().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1));
        let list_items: Vec<ListItem> = items
            .iter()
            .map(|(path, count)| ListItem::new(format!("{:<40} {}", path, count)))
            .collect();

        terminal.draw(|f| {
            let size = f.size();
            let title = format!("Code Volume - {}", current_dir);
            let block = Block::default().borders(Borders::ALL).title(title);
            let list = List::new(list_items).block(block).highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        if let Ok(true) = event::poll(std::time::Duration::from_millis(500)) {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break, // ESCキーで終了
                    KeyCode::Up => {
                        let i = match list_state.selected() {
                            Some(i) if i > 0 => i - 1,
                            _ => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = match list_state.selected() {
                            Some(i) if i < items.len().saturating_sub(1) => i + 1,
                            _ => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        if let Some(i) = list_state.selected() {
                            // 選択されたパスへ移動
                            current_dir = items[i].0.clone();
                            list_state.select(Some(0));
                        }
                    }
                    KeyCode::Backspace => {
                        // 親ディレクトリへ戻る（存在すれば）
                        if let Some(parent) = Path::new(&current_dir).parent() {
                            current_dir = parent.to_string_lossy().to_string();
                            list_state.select(Some(0));
                        }
                    }
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
