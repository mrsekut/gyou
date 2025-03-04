use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
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
}

#[derive(Debug)]
struct SizeItem {
    lines: u32,
    path: String,
}

impl App {
    pub fn new(current_dir: &str) -> Self {
        Self {
            list: SizeList { items: vec![] },
            dir: current_dir.to_string(),
            exit: false,
        }
    }

    // TODO: mut?
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    // TODO: inline
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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

impl App {
    fn render_app(&self, area: Rect, buf: &mut Buffer) {
        let title = self.title();
        let instructions = self.instructions();

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // let list_items: Vec<ListItem> = items
        //     .iter()
        //     .map(|item| {
        //         let style = if item.is_dir {
        //             Style::default().fg(Color::Blue)
        //         } else {
        //             Style::default().fg(Color::White)
        //         };
        //         ListItem::new(format!("{:>6} {}", item.count, item.path)).style(style)
        //     })
        //     .collect();

        // let counter_text = Text::from(vec![Line::from(vec![
        //     "Value: ".into(),
        //     self.counter.to_string().yellow(),
        // ])]);

        // Paragraph::new(counter_text)
        //     .centered()
        //     .block(block)
        //     .render(area, buf);
    }

    fn title(&self) -> Line {
        Line::from(format!("Code Size - {}", self.dir).bold())
    }

    fn instructions(&self) -> Line {
        Line::from(vec![
            "←: Parent".into(),
            "→: Child".into(),
            "↑↓: Select".into(),
            "q: Quit".into(),
        ])
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_app(area, buf)
    }
}
