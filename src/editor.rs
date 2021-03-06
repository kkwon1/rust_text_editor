use crate::Document;
use crate::Row;
use crate::Terminal;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Colors};
use std::env;

const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(_) = self.refresh_screen() {
                crash_and_burn();
            }

            if self.should_quit {
                break;
            }

            if let Err(_) = self.process_keypress() {
                crash_and_burn();
            }
        }
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document: document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::quit();
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        if let Event::Key(pressed_key) = Terminal::read_key()? {
            match (pressed_key.modifiers, pressed_key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) | (_, KeyCode::Esc) => {
                    Terminal::quit();
                    self.should_quit = true;
                    return Ok(());
                }
                (_, KeyCode::Up)
                | (_, KeyCode::Down)
                | (_, KeyCode::Left)
                | (_, KeyCode::Right)
                | (_, KeyCode::PageUp)
                | (_, KeyCode::PageDown)
                | (_, KeyCode::End)
                | (_, KeyCode::Home) => {
                    self.move_cursor(pressed_key);
                }
                _ => (),
            }
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key_event: KeyEvent) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Up) => y = y.saturating_sub(1),
            (_, KeyCode::Down) => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            (_, KeyCode::Left) => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            (_, KeyCode::Right) => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            (_, KeyCode::PageUp) => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            (_, KeyCode::PageDown) => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height as usize
                } else {
                    height
                }
            }
            (_, KeyCode::Home) => x = 0,
            (_, KeyCode::End) => x = width,
            _ => (),
        }

        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y };
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message()
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message: String = String::from("Welcome to Kevin's Text Editor!");
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let mut file_name = "[No Name]".to_string();

        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }

        status = format!("{} - {} lines", file_name, self.document.len());

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }

        status = format!("{}{}", status, line_indicator);

        status.truncate(width);
        Terminal::set_colors(Colors::new(STATUS_BG_COLOR, STATUS_FG_COLOR));
        println!("{}\r", status);
        Terminal::reset_colors();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
    }
}

fn crash_and_burn() {
    Terminal::clear_screen();
    panic!("Program crashed!");
}
