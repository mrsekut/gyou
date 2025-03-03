use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::ListItem,
    Terminal,
};
use std::{
    env,
    io::{self, stdout},
};
mod directory;
mod ui;
use directory::list_dir_items;
use ratatui::widgets::ListState;
use ui::{draw_ui, handle_event};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let root = if args.len() > 1 { &args[1] } else { "src" };
    let base = root.to_string();
    let mut current_dir = base.clone();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let mut terminal = setup_terminal()?;

    loop {
        let items = list_dir_items(&current_dir, &["ts", "tsx"])?;
        let list_items: Vec<ListItem> = items
            .iter()
            .map(|item| {
                let style = if item.is_dir {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{:>6} {}", item.count, item.path)).style(style)
            })
            .collect();

        draw_ui(&mut terminal, &current_dir, &mut list_state, &list_items)?;
        if let Some(next_dir) = handle_event(&current_dir, &base, &mut list_state, &items) {
            if next_dir == "__exit__" {
                break;
            }
            current_dir = next_dir;
            list_state.select(Some(0));
        }
    }

    restore_terminal(&mut terminal)
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(out);
    Terminal::new(backend)
}

fn restore_terminal<T: io::Write>(terminal: &mut Terminal<CrosstermBackend<T>>) -> io::Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
