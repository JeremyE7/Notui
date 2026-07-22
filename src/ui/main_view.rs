use std::path::Path;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::{app::App, mode::Mode, notes::read_note_content};

pub fn render_main_view(f: &mut Frame, app: &mut App, vault: &Path) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .spacing(2)
        .split(f.area());

    // --- Panel izquierdo: la lista (esto ya lo tenías) ---
    let items: Vec<ListItem> = app
        .notes
        .iter()
        .map(|n| ListItem::new(n.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().yellow())
                .title_bottom("Notas"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightYellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.selected));

    f.render_stateful_widget(list, chunks[0], &mut state);

    // --- Panel derecho: preview del contenido ---
    let content = if let Some(note) = app.notes.get(app.selected) {
        read_note_content(Path::new("vault"), note)
    } else {
        String::new()
    };

    if let Mode::Normal = &app.mode {
        let preview = Paragraph::new(content).block(Block::default());
        f.render_widget(preview, chunks[1]);
    }

    if let Mode::EditNote(_input) = &app.mode {
        f.render_widget(&app.text_area, chunks[1]);
    }
}
