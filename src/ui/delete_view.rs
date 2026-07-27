use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, mode::AppMode};

pub fn render_delete_note_view(f: &mut Frame, app: &mut App) {
    if let AppMode::DeleteNote(note) = &app.mode {
        let popup_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(45),
                Constraint::Length(3),
                Constraint::Percentage(45),
            ])
            .split(f.area())[1];

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(popup_area)[1];

        let text_to_display: String = format!("¿Eliminar {}?", note);

        let text = Paragraph::new(text_to_display).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightYellow)
                .title("(d=eliminar, Esc=cancelar)"),
        );

        f.render_widget(ratatui::widgets::Clear, popup_area); // limpia el fondo antes de dibujar encima
        f.render_widget(text, popup_area);
    }
}
