use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

use crate::{app::App, mode::AppMode};

pub fn render_status_bar(f: &mut Frame, app: &mut App, area: Rect) {
    // Barra inferior
    let mode_text = match &app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::EditNote(_) => "INSERT",
        AppMode::DeleteNote(_) => "DELETE",
        AppMode::NewNote(_) => "NEW",
    };

    let status_text = format!(
        " Modo: {}  |  Nota: {}/{} ",
        mode_text,
        app.selected + 1,
        app.notes.len()
    );

    let status_bar = Paragraph::new(status_text).style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(status_bar, area);
}
