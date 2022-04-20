use crate::Terminal;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
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
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });

        if self.should_quit {
            Terminal::quit();
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
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
                    Ok(())
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
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    fn move_cursor(&mut self, key_event: KeyEvent) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Up) => y = y.saturating_sub(1),
            (_, KeyCode::Down) => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            (_, KeyCode::Left) => x = x.saturating_sub(1),
            (_, KeyCode::Right) => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            (_, KeyCode::PageUp) => y = 0,
            (_, KeyCode::PageDown) => y = height,
            (_, KeyCode::Home) => x = 0,
            (_, KeyCode::End) => x = width,
            _ => (),
        }

        self.cursor_position = Position { x, y };
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
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
}

fn crash_and_burn() {
    Terminal::clear_screen();
    panic!("Program crashed!");
}
