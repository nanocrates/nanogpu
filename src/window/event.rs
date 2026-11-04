use super::key::Key;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowEvent {
    Close,
    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),
    KeyRepeat(KeyEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
}

