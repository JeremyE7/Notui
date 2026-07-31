use std::usize;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, mode::AppMode};

pub fn render_confirm_view(f: &mut Frame, app: &mut App) {
    if let AppMode::Confirm(index) = &app.mode {
        let index = *index;
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
        let mut text_to_display = String::new();
        if let Some(note) = app.notes.get(index) {
            text_to_display = format!(
                "Las siguiente nota tienen cambios sin guardar: {}. \n ¿Desea salir?",
                note.name
            );
        }

        let text = Paragraph::new(text_to_display).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightYellow)
                .title("(d=Cancelar, Esc/q=Salir)"),
        );

        f.render_widget(ratatui::widgets::Clear, popup_area); // limpia el fondo antes de dibujar encima
        f.render_widget(text, popup_area);
    }
}
