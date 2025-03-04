use crate::directory::DirItem;
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
    exit: bool,
}

#[derive(Debug)]
struct SizeList {
    items: Vec<DirItem>,
    state: ListState,
}

impl App {
    pub fn new(cur_dir: String, base_dir: String, items: Vec<DirItem>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self {
            list: SizeList { items, state },
            cur_dir,
            base_dir,
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
