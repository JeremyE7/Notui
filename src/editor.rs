use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui_textarea::{CursorMove, TextArea};

use crate::mode::EditorMode;

pub struct Editor {
    pub text_area: TextArea<'static>,
    pub editor_mode: EditorMode,
}

pub enum EditorAction {
    None,
    Save,
    Quit,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text_area: TextArea::default(),
            editor_mode: EditorMode::Normal,
        }
    }
    pub fn handle_visual_mode(&mut self, key: KeyEvent) -> EditorAction {
        match key.code {
            KeyCode::Esc => {
                self.editor_mode = EditorMode::Normal;
                self.text_area.cancel_selection();
                return EditorAction::None;
            }
            KeyCode::Char('l') => self.text_area.move_cursor(CursorMove::Forward),
            KeyCode::Char('h') => self.text_area.move_cursor(CursorMove::Back),
            KeyCode::Char('k') => self.text_area.move_cursor(CursorMove::Up),
            KeyCode::Char('j') => self.text_area.move_cursor(CursorMove::Down),
            KeyCode::Char('e') => self.text_area.move_cursor(CursorMove::WordEnd),
            KeyCode::Char('b') => self.text_area.move_cursor(CursorMove::WordBack),
            KeyCode::Char('y') => self.text_area.copy(),

            _ => {}
        }
        return EditorAction::None;
    }
    pub fn handle_insert_mode(&mut self, key: KeyEvent) -> EditorAction {
        match key.code {
            KeyCode::Esc => {
                self.editor_mode = EditorMode::Normal;
                return EditorAction::None;
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return EditorAction::Save;
            }
            _ => {
                self.text_area.input(key);
                return EditorAction::None;
            }
        }
    }
    pub fn handle_command_mode(&mut self, key: KeyEvent) -> EditorAction {
        match key.code {
            _ => {}
        }
        return EditorAction::None;
    }
    pub fn handle_normal_mode(&mut self, key: KeyEvent) -> EditorAction {
        match key.code {
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return EditorAction::Save;
            }
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.text_area.move_cursor(CursorMove::End);
                self.text_area.insert_newline();
                self.text_area.insert_str("[ ] ");
            }
            KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.text_area.redo();
            }
            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.text_area.select_all();
            }

            KeyCode::Esc => {
                return EditorAction::Quit;
            }
            KeyCode::Char('l') => self.text_area.move_cursor(CursorMove::Forward),
            KeyCode::Char('h') => self.text_area.move_cursor(CursorMove::Back),
            KeyCode::Char('k') => self.text_area.move_cursor(CursorMove::Up),
            KeyCode::Char('j') => self.text_area.move_cursor(CursorMove::Down),
            KeyCode::Char('e') => self.text_area.move_cursor(CursorMove::WordEnd),
            KeyCode::Char('b') => self.text_area.move_cursor(CursorMove::WordBack),
            KeyCode::Char('p') => {
                self.text_area.paste();
            }
            KeyCode::Char('v') => {
                self.text_area.start_selection();
                self.editor_mode = EditorMode::Visual
            }
            KeyCode::Char('u') => {
                self.text_area.undo();
            }
            KeyCode::Char('a') => {
                self.text_area.move_cursor(CursorMove::Forward);
                self.editor_mode = EditorMode::Insert
            }
            KeyCode::Char('i') => {
                self.text_area.move_cursor(CursorMove::Back);
                self.editor_mode = EditorMode::Insert;
            }
            KeyCode::Char('o') => {
                self.text_area.move_cursor(CursorMove::End);
                self.text_area.insert_newline();
                self.editor_mode = EditorMode::Insert
            }

            _ => {}
        }
        return EditorAction::None;
    }
}
