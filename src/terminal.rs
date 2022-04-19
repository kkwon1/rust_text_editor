use crossterm::event;
use crossterm::event::Event;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{self, stdout, Write};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        // Enable raw mode so that characters are directly read from and written to the device
        terminal::enable_raw_mode().expect("Failed to enable Raw mode");
        let size = terminal::size().unwrap();
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn quit() {
        Terminal::clear_screen();
        terminal::disable_raw_mode().expect("Failed to disable Raw mode");
        println!("Goodbye!");
    }

    pub fn clear_screen() {
        println!("{}", terminal::Clear(terminal::ClearType::All));
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        std::io::stdout().execute(cursor::MoveTo(x - 1, y - 1)).ok();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn read_key() -> Result<Event, std::io::Error> {
        event::read()
    }

    pub fn cursor_hide() {
        stdout().execute(cursor::DisableBlinking).ok();
    }

    pub fn cursor_show() {
        stdout().execute(cursor::EnableBlinking).ok();
    }

    pub fn clear_current_line() {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .ok();
    }
}
