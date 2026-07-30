use crate::editor::Editor;
use crate::mode::AppMode;
use crate::notes::Note;

use ratatui_notifications::{
    Anchor, Animation, Level, Notification, Notifications, SizeConstraint,
};
use std::path::Path;

use std::{fs, io};

// --- Estado de la app ---
pub struct App {
    pub notes: Vec<Note>,
    pub selected: usize, // índice de la nota seleccionada
    pub mode: AppMode,   // nuevo campo
    pub notifications: Notifications,
    pub editor: Editor,
}

impl App {
    pub fn new(vault: &Path) -> io::Result<Self> {
        let notes = App::list_notes(vault)?;
        Ok(App {
            notes,
            selected: 0,
            mode: AppMode::Normal,
            notifications: Notifications::new(),
            editor: Editor::new(),
        })
    }

    pub fn next(&mut self) {
        if !self.notes.is_empty() {
            self.selected = (self.selected + 1) % self.notes.len();
        }
    }

    pub fn get_selected_note_mut(&mut self) -> &mut Note {
        self.notes
            .get_mut(self.selected)
            .expect("La nota seleccionada debe existir")
    }

    pub fn previous(&mut self) {
        if !self.notes.is_empty() {
            self.selected = if self.selected == 0 {
                self.notes.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn add_notification(&mut self, kind: Level, title: String, text: String) {
        let notif = Notification::new(text)
            .title(title)
            .level(kind)
            .anchor(Anchor::BottomRight)
            .animation(Animation::Fade)
            .max_size(SizeConstraint::Absolute(30), SizeConstraint::Absolute(1))
            .build()
            .unwrap();

        self.notifications.add(notif).unwrap();
    }

    pub fn list_notes(vault: &Path) -> io::Result<Vec<Note>> {
        let mut notes = Vec::new();
        for entry in fs::read_dir(vault)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let note = Note::new(name.to_string());
                    notes.push(note);
                }
            }
        }
        notes.sort();
        Ok(notes)
    }
}
