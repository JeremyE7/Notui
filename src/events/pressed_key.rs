use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PressedKey {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl From<KeyEvent> for PressedKey {
    fn from(key: KeyEvent) -> Self {
        Self {
            code: key.code,
            modifiers: key.modifiers,
        }
    }
}
