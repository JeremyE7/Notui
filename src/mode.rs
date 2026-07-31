pub enum AppMode {
    Normal,            // navegando la lista (como ahora)
    NewNote(String),   // escribiendo el título de una nota nueva
    DeleteNote(usize), // borrando una nota
    EditNote(usize),
    Confirm(usize),
}

pub enum EditorMode {
    Visual,
    Normal,
    Insert,
    Command,
}
