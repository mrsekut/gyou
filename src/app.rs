use crate::directory::{list_dir_items, DirItem};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct App {
    cur_dir: String,
    base_dir: String,
    list: SizeList,
    exts: Vec<String>,
    exit: bool,
}

#[derive(Debug)]
struct SizeList {
    items: Vec<DirItem>,
    state: ListState,
}

// TODO: fix:unwrap, clone
impl App {
    // TODO: args
    pub fn new(base_dir: &str, exts: &Vec<String>) -> Self {
        let items = list_dir_items(&base_dir, exts).unwrap();
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self {
            list: SizeList { items, state },
            cur_dir: base_dir.to_string(),
            base_dir: base_dir.to_string(),
            exts: exts.iter().map(|s| s.to_string()).collect(),
            exit: false,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Up => self.select_prev(),
            KeyCode::Down => self.select_next(),
            KeyCode::Left => self.move_to_parent(),
            KeyCode::Right => self.move_to_child(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn select_prev(&mut self) {
        self.list.state.select_previous();
    }

    fn select_next(&mut self) {
        self.list.state.select_next();
    }

    fn move_to_parent(&mut self) {
        todo!()
    }

    fn move_to_child(&mut self) {
        if let Some(i) = self.list.state.selected() {
            let item = &self.list.items[i];
            if item.is_dir {
                self.cur_dir = item.path.clone();
                self.list.items = list_dir_items(&item.path, &self.exts).unwrap();
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);

        self.render_list(main_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl App {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let title = format!("Code Volume - {}", self.cur_dir);
        let block = Block::new().borders(Borders::ALL).title(title);

        let items: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|item| {
                let color = if item.is_dir {
                    Color::Blue
                } else {
                    Color::White
                };
                let style = Style::default().fg(color);
                ListItem::new(format!("{:>6} {}", item.count, item.path)).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("←: Parent   →: Child   ↑↓: Select   q: Quit")
            .block(Block::default().borders(Borders::ALL).title("Instructions"))
            .render(area, buf)
    }
}
