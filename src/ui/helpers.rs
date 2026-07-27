use std::io::{self};

use ratatui::style::{Modifier, Style};

use crate::{
    app::App,
    mode::{AppMode, EditorMode},
};
pub fn update_cursor_style(app: &mut App) -> io::Result<()> {
    let cursor_style = match (&app.mode, &app.editor.editor_mode) {
        (AppMode::EditNote(_), EditorMode::Insert) => Style::default()
            .add_modifier(Modifier::RAPID_BLINK)
            .on_yellow()
            .black(),

        // Cursor de bloque en los demás modos
        _ => Style::default().add_modifier(Modifier::REVERSED),
    };

    app.editor.text_area.set_cursor_style(cursor_style);

    Ok(())
}
