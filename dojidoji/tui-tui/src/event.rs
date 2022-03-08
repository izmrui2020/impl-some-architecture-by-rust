//
use std::sync::mpsc::Receiver;

use termion::event::{Key};

pub enum Event {
    Input(Key),
    Tick,
}

pub struct Events {
    rx: Receiver<Key>
}