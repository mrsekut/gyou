use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    CompletedFrame, Terminal,
};
use std::{
    env, fs,
    io::{self, stdout},
    path::Path,
};
use walkdir::WalkDir;

// 新規: 現在のディレクトリ内のエントリ（ディレクトリまたはファイル）の一覧を返す
// 戻り値のタプルは (パス, 行数, is_directory)
fn list_dir_items(root: &str, exts: &[&str]) -> io::Result<Vec<(String, usize, bool)>> {
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
    // 降順ソート（行数で並び替え）
    results.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(results)
}

// ターミナルのセットアップ
fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(out);
    Terminal::new(backend)
}

// ターミナルのリセット
fn restore_terminal<T: io::Write>(terminal: &mut Terminal<CrosstermBackend<T>>) -> io::Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// UI描画: 現在のディレクトリとそのエントリの行数リストを表示
fn draw_ui<'a>(
    terminal: &'a mut Terminal<CrosstermBackend<io::Stdout>>,
    current_dir: &'a str,
    state: &'a mut ListState,
    list_items: &'a [ListItem<'a>],
) -> Result<CompletedFrame<'a>, std::io::Error> {
    let title = format!("Code Volume - {}", current_dir);
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().borders(Borders::ALL).title(title);
        let list = List::new(list_items.iter().cloned().collect::<Vec<_>>())
            .block(block)
            .highlight_symbol(">> ");
        let _ = f.render_stateful_widget(list, size, state);
    })
}

// イベント処理: 左右キーにより、親/子ディレクトリへの移動を行う（ファイル選択時は移動しない）
fn handle_event(
    current_dir: &str,
    state: &mut ListState,
    items: &[(String, usize, bool)],
) -> Option<String> {
    if let Event::Key(key) = event::read().ok()? {
        match key.code {
            KeyCode::Left => {
                if let Some(parent) = Path::new(current_dir).parent() {
                    return Some(parent.to_string_lossy().to_string());
                }
            }
            // 右矢印で、選択中がディレクトリの場合のみ子ディレクトリへ移動
            KeyCode::Right => {
                if let Some(i) = state.selected() {
                    if items.get(i).map(|v| v.2).unwrap_or(false) {
                        return Some(items[i].0.clone());
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
            // ESCで終了
            KeyCode::Esc => return Some("__exit__".to_string()),
            _ => {}
        }
    }
    None
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let root = if args.len() > 1 { &args[1] } else { "src" };
    let mut current_dir = root.to_string();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let mut terminal = setup_terminal()?;

    loop {
        // ファイルも含めたエントリを取得
        let items = list_dir_items(&current_dir, &["ts", "tsx"])?;
        // ListItem の表示に、[DIR]または[FILE]のタグを追加
        let list_items: Vec<ListItem> = items
            .iter()
            .map(|(p, cnt, is_dir)| {
                let tag = if *is_dir { "[DIR]" } else { "[FILE]" };
                ListItem::new(format!("{:<40} {} {}", p, cnt, tag))
            })
            .collect();

        draw_ui(&mut terminal, &current_dir, &mut list_state, &list_items)?;
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Some(next_dir) = handle_event(&current_dir, &mut list_state, &items) {
                if next_dir == "__exit__" {
                    break;
                }
                current_dir = next_dir;
                list_state.select(Some(0));
            }
        }
    }

    restore_terminal(&mut terminal)
}
