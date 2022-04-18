use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::io;
use std::io::Read;

fn main() {
    // Enable raw mode so that characters are directly read from and written to the device
    terminal::enable_raw_mode().expect("Could not enable Raw mode");

    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::CONTROL,
                } => {
                    crossterm::terminal::disable_raw_mode().ok();
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("{:?} \r", event);
                }
            }
        }
    }

    // for b in io::stdin().bytes() {
    //     match b {
    //         Ok(b) => {
    //             let c = b as char;

    //             // differentiating between control characters and non-control characters
    //             // https://www.asciitable.com/
    //             if c.is_control() {
    //                 println!("{:?} \r", b);
    //             } else {
    //                 println!("{:?} ({})\r", b, c);
    //             }

    //             if b == to_ctrl_byte('q') {
    //                 crossterm::terminal::disable_raw_mode().ok();
    //                 break;
    //             }
    //         }
    //         Err(err) => crash_and_burn(err),
    //     }
    // }
}

// control characters are represented by bytes from 0 - 31.
// mask with the lowest 5 bits to get the control character representation.
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn crash_and_burn(e: std::io::Error) {
    panic!("{}", e);
}
