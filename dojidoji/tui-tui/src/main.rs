//
use termion::event::Key;
use termion::input::TermRead;
use anyhow::Result;
use std::sync::mpsc;
use std::thread;
use std::io;

mod utils;
mod draw;
mod event;

use self::event::Event;
use self::draw::draw::Termondayo;


fn main() -> Result<()> {
    let mut termon = Termondayo::new();

    termon.terminal.clear()?;
    termon.terminal.hide_cursor()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let stdin = io::stdin();

            for ev in stdin.keys() {
                match ev {
                    Ok(key) => {
                        tx.send(Event::Input(key)).unwrap();
                    },
                    Err(e) => {
                        println!("failed: {}", e);
                        break;
                    }
                }
            }
        }
    });

    loop {
        match termon.draw() {
            Ok(_) => {
                
            }
            Err(e) => {
                println!("failed at draw: {}", e);
                break;
            }
        }

        match rx.recv() {
            Ok(event) => {
                match event {
                    Event::Input(key) => match key {
                        Key::Char('q') => {
                            println!("push q");
                            break;
                        },
                        Key::Backspace => todo!(),
                        Key::Left => todo!(),
                        Key::Right => todo!(),
                        Key::Up => todo!(),
                        Key::Down => todo!(),
                        Key::Home => todo!(),
                        Key::End => todo!(),
                        Key::PageUp => todo!(),
                        Key::PageDown => todo!(),
                        Key::BackTab => todo!(),
                        Key::Delete => todo!(),
                        Key::Insert => todo!(),
                        Key::F(_) => todo!(),
                        Key::Alt(_) => todo!(),
                        Key::Ctrl(_) => todo!(),
                        Key::Null => todo!(),
                        Key::Esc => todo!(),
                        _ => todo!(),
                    }
                    Event::Tick => todo!(),
                }
            }
            Err(e) => {
                println!("failed recv(): {}", &e);
                break;
            }
        }
    }

    Ok(())
}
