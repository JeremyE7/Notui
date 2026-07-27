use ratatui::{
    Frame,
    layout::Rect,
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

    let status_bar = Paragraph::new(status_text).style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(status_bar, area);
}
