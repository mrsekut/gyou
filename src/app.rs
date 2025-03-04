use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};
use std::io::{self};

#[derive(Debug)]
pub struct App {
    dir: String,
    list: SizeList,
    exit: bool,
}

#[derive(Debug)]
struct SizeList {
    items: Vec<SizeItem>,
    state: ListState,
}

#[derive(Debug)]
struct SizeItem {
    is_dir: bool,
    count: u32,
    path: String,
}

impl App {
    pub fn new(current_dir: &str) -> Self {
        let state = ListState::default();
        Self {
            list: SizeList {
                items: vec![],
                state,
            },
            dir: current_dir.to_string(),
            exit: false,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }

    // fn handle_key_event(&mut self, key_event: KeyEvent) {
    //     match key_event.code {
    //         KeyCode::Char('q') => self.exit(),
    //         KeyCode::Left => self.decrement_counter(),
    //         KeyCode::Right => self.increment_counter(),
    //         _ => {}
    //     }
    // }

    fn exit(&mut self) {
        self.exit = true;
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
        let title = format!("Code Volume - {}", self.dir);
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
