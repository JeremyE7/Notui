use ratatui_textarea::TextArea;

use crate::mode::Mode;
use crate::notes::list_notes;

use ratatui_notifications::{
    Anchor, Animation, Level, Notification, Notifications, SizeConstraint,
};
use std::io;
use std::path::Path;

// --- Estado de la app ---
pub struct App {
    pub notes: Vec<String>,
    pub selected: usize, // índice de la nota seleccionada
    pub mode: Mode,      // nuevo campo
    pub text_area: TextArea<'static>,
    pub notifications: Notifications,
}

impl App {
    pub fn new(vault: &Path) -> io::Result<Self> {
        let notes = list_notes(vault)?;
        Ok(App {
            notes,
            selected: 0,
            mode: Mode::Normal,
            text_area: TextArea::default(),
            notifications: Notifications::new(),
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
