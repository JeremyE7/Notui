use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

use crate::{
    app::App,
    mode::{AppMode, EditorMode},
};

pub fn render_status_bar(f: &mut Frame, app: &mut App, area: Rect) {
    // Barra inferior
    let mode_text = match &app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::EditNote(_) => "EDIT",
        AppMode::DeleteNote(_) => "DELETE",
        AppMode::NewNote(_) => "NEW",
    };

    let editor_mode_text = match &app.editor.editor_mode {
        EditorMode::Normal => "NORMAL",
        EditorMode::Insert => "INSERT",
        EditorMode::Visual => "VISUAL",
        EditorMode::Command => "COMMAND",
    };

    let current_note = if app.notes.is_empty() {
        0
    } else {
        app.selected + 1
    };

    let status_text = if matches!(&app.mode, AppMode::EditNote(_)) {
        format!(
            " Modo: {}  |  Editor: {}  |  Nota: {}/{} ",
            mode_text,
            editor_mode_text,
            current_note,
            app.notes.len()
        )
    } else {
        format!(
            " Modo: {}  |  Nota: {}/{} ",
            mode_text,
            current_note,
            app.notes.len()
        )
    };

    let buffer_key = &app.editor.key_buffer.last();
    let buffer_text = match buffer_key {
        Some(key) => format!("{:?}", key.code),
        None => String::new(),
    };

    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),     // Texto principal ocupa el espacio disponible
            Constraint::Length(12), // Espacio reservado para la tecla
        ])
        .split(area);

    let status_style = Style::default()
        .bg(Color::Yellow)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);

    let status_bar = Paragraph::new(status_text).style(status_style);

    let key_bar = Paragraph::new(buffer_text)
        .style(status_style)
        .alignment(Alignment::Right);

    f.render_widget(status_bar, status_chunks[0]);
    f.render_widget(key_bar, status_chunks[1]);
}
