mod main_view;
use ratatui::Frame;

use crate::app::App;
use crate::mode::Mode;
use crate::notes::read_note_content;
use std::path::Path;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn draw(f: &mut Frame, app: &mut App, vault: &Path) {
    // Divide la pantalla en 2 columnas: 30% lista, 70% preview
    main_view::render_main_view(f, app, vault);
    // --- Overlay: input de nueva nota ---
    if let Mode::NewNote(input) = &app.mode {
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

    if let Mode::DeleteNote(note) = &app.mode {
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

    app.notifications.render(f, f.area());
}
