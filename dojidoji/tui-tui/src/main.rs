//
use termion::event::Key;
use termion::input::TermRead;
use anyhow::Result;
use tokio::sync::mpsc;
use std::io;

mod utils;
mod draw;
mod event;
mod data_center;

use self::data_center::DataCenter;
use self::event::Event;
use self::draw::draw::Termondayo;
use self::draw::sub::SubCenter;


#[tokio::main]
async fn main() -> Result<()> {

    println!("start main");

    let (tx, rx) = mpsc::channel(100);

    let mut data_center = DataCenter::new(tx.clone());

    let mut termon = Termondayo::new();
    let sub_center = SubCenter::new(rx);

    termon.terminal.clear()?;
    termon.terminal.hide_cursor()?;

    let task1 = tokio::spawn(async move {
        data_center.send_data().await;
    });

    let (mut event_tx, mut event_rx) = mpsc::channel(1);
    tokio::spawn(async move {
        loop {
            let stdin = io::stdin();

            for ev in stdin.keys() {
                match ev {
                    Ok(key) => {
                        event_tx.send(Event::Input(key)).await;
                    },
                    Err(e) => {
                        println!("failed: {}", e);
                        break;
                    }
                }
            }
        }
    });

    let task2 = tokio::spawn(async move {
        loop {
            match termon.draw().await {
                Ok(_) => {
                    
                }
                Err(e) => {
                    println!("failed at draw: {}", e);
                    break;
                }
            }
    
            match event_rx.recv().await {
                Some(event) => {
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
                None => {
                    println!("failed recv():");
                    break;
                }
            }
        }
    });

    
    

    task1.await?;
    task2.await?;

    Ok(())
}
