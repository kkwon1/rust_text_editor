use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::Write;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        // Enable raw mode so that characters are directly read from and written to the device
        terminal::enable_raw_mode().expect("Failed to enable Raw mode");

        loop {
            if let Err(_) = self.refresh_screen() {
                crash_and_burn();
            }

            if self.should_quit {
                break;
            } else {
                self.draw_rows();
                std::io::stdout().execute(cursor::MoveTo(0, 0)).ok();
            }

            if let Err(_) = self.process_keypress() {
                crash_and_burn();
            }
        }
    }

    pub fn default() -> Self {
        Self { should_quit: false }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        std::io::stdout()
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?
            .flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        if let Event::Key(pressed_key) = read_key()? {
            match pressed_key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::CONTROL,
                }
                | KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    exit_gracefully();
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
        for _ in 0..24 {
            println!("~\r");
        }
    }
}

fn read_key() -> Result<Event, std::io::Error> {
    event::read()
}

fn exit_gracefully() {
    crossterm::terminal::disable_raw_mode().expect("Failed to disable Raw mode");
    println!("Goodbye!");
}

fn crash_and_burn() {
    println!("{}", terminal::Clear(terminal::ClearType::All));
    panic!("Program crashed!");
}
