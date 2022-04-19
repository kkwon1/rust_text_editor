use crate::Terminal;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();

        if self.should_quit {
            Terminal::quit();
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        if let Event::Key(pressed_key) = Terminal::read_key()? {
            match pressed_key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::CONTROL,
                }
                | KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    Terminal::quit();
                    self.should_quit = true;
                    Ok(())
                }
                _ => {
                    println!("{:?} \r", pressed_key);
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                println!("Welcome to Kevin's Rusty Text Editor!");
            } else {
                println!("~\r");
            }
        }
    }
}

fn crash_and_burn() {
    Terminal::clear_screen();
    panic!("Program crashed!");
}
