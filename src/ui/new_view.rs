use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, mode::AppMode};

pub fn render_new_note_view(f: &mut Frame, app: &mut App) {
    if let AppMode::NewNote(input) = &app.mode {
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

        let input_box = Paragraph::new(input.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::LightYellow)
                .title("Nueva nota (Enter=crear, Esc=cancelar)"),
        );

        f.render_widget(ratatui::widgets::Clear, popup_area); // limpia el fondo antes de dibujar encima
        f.render_widget(input_box, popup_area);
    }
}
