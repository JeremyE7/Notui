use crate::editor::Editor;
use crate::mode::{AppMode, EditorMode};
use crate::notes::{list_notes, save_note};

use ratatui_notifications::{
    Anchor, Animation, Level, Notification, Notifications, SizeConstraint,
};
use std::path::Path;

use std::io;

// --- Estado de la app ---
pub struct App {
    pub notes: Vec<String>,
    pub selected: usize, // índice de la nota seleccionada
    pub mode: AppMode,   // nuevo campo
    pub notifications: Notifications,
    pub editor: Editor,
}

impl App {
    pub fn new(vault: &Path) -> io::Result<Self> {
        let notes = list_notes(vault)?;
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

    pub fn previous(&mut self) {
        if !self.notes.is_empty() {
            self.selected = if self.selected == 0 {
                self.notes.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn save(&mut self, note: String) {
        let text: String = self.editor.text_area.lines().join("\n");
        let path = Path::new("vault").join(note);
        let _ = save_note(&path, &text);
        self.add_notification(
            Level::Info,
            "Exito".to_string(),
            "Archivo guardado".to_string(),
        );
        self.editor.editor_mode = EditorMode::Normal
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
}
