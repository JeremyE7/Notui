mod pressed_key;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Color, Style};
use ratatui_notifications::Level;
use ratatui_textarea::TextArea;

use crate::app::App;
use crate::editor::EditorAction;
use crate::mode::{AppMode, EditorMode};
use crate::notes::{Note, NoteState};
use crate::ui;
pub use pressed_key::PressedKey;
use std::io;
use std::path::Path;

pub fn handle_key(app: &mut App, key: KeyEvent) -> io::Result<bool> {
    app.editor.key_buffer.push(key.into());
    match &mut app.mode {
        AppMode::Normal => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if let Some(index) = app
                    .notes
                    .iter()
                    .position(|note| note.state == NoteState::Edited)
                {
                    app.mode = AppMode::Confirm(index);
                } else {
                    return Ok(true);
                }
            }
            KeyCode::Char('j') | KeyCode::Down => app.next(),
            KeyCode::Char('k') | KeyCode::Up => app.previous(),
            KeyCode::Char('n') => {
                app.mode = AppMode::NewNote(String::new()); // entra a modo creación
            }
            KeyCode::Char('e') => {
                if let Some(note) = app.notes.get_mut(app.selected) {
                    note.load(Path::new("vault"))?;
                    app.editor.text_area = TextArea::new(note.text.clone());
                    app.editor.text_area.set_cursor_line_style(Style::default());

                    app.editor.text_area.set_cursor_style(Style::default());

                    let style = Style::default().fg(Color::DarkGray);
                    app.editor.text_area.set_line_number_style(style);

                    app.mode = AppMode::EditNote(app.selected);
                    app.editor.editor_mode = EditorMode::Normal;
                }
            }

            KeyCode::Char('d') => {
                app.mode = AppMode::DeleteNote(app.selected);
            }
            _ => {}
        },
        AppMode::NewNote(input) => match key.code {
            KeyCode::Esc => {
                app.mode = AppMode::Normal; // cancelar
            }
            KeyCode::Enter => {
                let title = input.clone();
                let filename = Note::create(Path::new("vault"), &title)?;
                app.notes = App::list_notes(Path::new("vault"))?; // recarga la lista
                app.selected = app.notes.iter().position(|n| n == &filename).unwrap_or(0);
                app.add_notification(
                    Level::Warn,
                    "Exito".to_string(),
                    "Archivo creado".to_string(),
                );
                app.mode = AppMode::Normal;
            }
            KeyCode::Char(c) => {
                input.push(c); // agrega el caracter al título
            }
            KeyCode::Backspace => {
                input.pop(); // borra el último caracter
            }
            _ => {}
        },
        AppMode::DeleteNote(index) => match key.code {
            KeyCode::Esc => {
                app.mode = AppMode::Normal; // cancelar
            }
            KeyCode::Char('d') => {
                let index = *index;
                let path = Path::new("vault");

                if let Some(note) = app.notes.get(index) {
                    note.delete(path)?;
                }

                app.notes = App::list_notes(Path::new("vault"))?;
                if app.notes.len() == 0 {
                    app.selected = 0;
                } else if app.selected >= app.notes.len() {
                    app.selected = app.notes.len() - 1;
                }
                app.add_notification(
                    Level::Error,
                    "Exito".to_string(),
                    "Archivo eliminado".to_string(),
                );
                app.mode = AppMode::Normal; // cancelar
            }
            _ => {}
        },
        AppMode::EditNote(index) => {
            let index = *index;

            let action = match app.editor.editor_mode {
                EditorMode::Visual => app.editor.handle_visual_mode(key),
                EditorMode::Insert => app.editor.handle_insert_mode(key),
                EditorMode::Normal => app.editor.handle_normal_mode(key),
                EditorMode::Command => app.editor.handle_command_mode(key),
            };

            match action {
                EditorAction::Quit => app.mode = AppMode::Normal,
                EditorAction::None => {}
                EditorAction::Leave => {
                    let text = app.editor.text_area.lines().to_vec();
                    if let Some(note) = app.notes.get_mut(index) {
                        note.save_on_memory(text);
                    }
                }
                EditorAction::Save => {
                    let path = Path::new("vault");
                    if let Some(note) = app.notes.get_mut(index) {
                        note.save(path)?;
                    }
                }
                EditorAction::Notify(kind, title, messagge) => {
                    app.add_notification(kind, title, messagge);
                }
            }
        }
        AppMode::Confirm(_) => match key.code {
            KeyCode::Esc => {
                app.mode = AppMode::Normal; // cancelar
            }
            KeyCode::Char('q') => {
                app.mode = AppMode::Normal; // cancelar
                return Ok(true);
            }
            _ => {}
        },
    }
    ui::update_cursor_style(app)?;
    return Ok(false);
}
