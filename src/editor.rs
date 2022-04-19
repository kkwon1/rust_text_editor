use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        // Enable raw mode so that characters are directly read from and written to the device
        terminal::enable_raw_mode().expect("Failed to enable Raw mode");

        loop {
            if let Err(_) = self.process_keypress() {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self {}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
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
                    Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Exit"))
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
}

fn read_key() -> Result<Event, std::io::Error> {
    event::read()
}

fn exit_gracefully() {
    crossterm::terminal::disable_raw_mode().expect("Failed to disable Raw mode");
    println!("Goodbye!");
}
