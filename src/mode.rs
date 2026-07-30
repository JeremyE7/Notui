use crate::notes::Note;

pub enum AppMode {
    Normal,           // navegando la lista (como ahora)
    NewNote(String),  // escribiendo el título de una nota nueva
    DeleteNote(Note), // borrando una nota
    EditNote(Note),
}

pub enum EditorMode {
    Visual,
    Normal,
    Insert,
    Command,
}
