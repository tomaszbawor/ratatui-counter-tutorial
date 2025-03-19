use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    menu_items: Vec<&'static str>,
    active_menu_item: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: Default::default(),
            menu_items: vec!["One", "Two", "Three"],
            active_menu_item: 0,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up | KeyCode::Char('j') => self.menu_up(),
            KeyCode::Down | KeyCode::Char('k') => self.menu_down(),
            _ => {}
        }
    }

    fn menu_up(&mut self) {
        if self.active_menu_item == 0 {
            self.active_menu_item = self.menu_items.len() - 1;
        } else {
            self.active_menu_item -= 1;
        }
    }

    fn menu_down(&mut self) {
        if self.active_menu_item == (self.menu_items.len() - 1) {
            self.active_menu_item = 0;
        } else {
            self.active_menu_item += 1;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Test Application Main Menu ".bold());

        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let menu_lines: Vec<Line> = self
            .menu_items
            .iter()
            .enumerate()
            .map(|(i, menu_item)| {
                let mut line = Line::from(*menu_item);
                if self.active_menu_item == i {
                    line = line.bold().red();
                }
                line
            })
            .collect();

        let menu_text = Text::from(menu_lines);

        Paragraph::new(menu_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();

        app.handle_key_event(KeyCode::Down.into());
        assert_eq!(app.active_menu_item, 1);

        app.handle_key_event(KeyCode::Up.into());
        assert_eq!(app.active_menu_item, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
