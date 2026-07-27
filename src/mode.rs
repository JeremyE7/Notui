pub enum AppMode {
    Normal,             // navegando la lista (como ahora)
    NewNote(String),    // escribiendo el título de una nota nueva
    DeleteNote(String), // borrando una nota
    EditNote(String),
}

pub enum EditorMode {
    Visual,
    Normal,
    Insert,
    Command,
}
