mod confirm_view;
mod delete_view;
mod helpers;
mod main_view;
mod new_view;
mod status_bar;
use ratatui::Frame;

use crate::app::App;
pub use helpers::update_cursor_style;

pub fn draw(f: &mut Frame, app: &mut App) {
    // Divide la pantalla en 2 columnas: 30% lista, 70% preview
    main_view::render_main_view(f, app);
    // --- Overlay: input de nueva nota ---
    new_view::render_new_note_view(f, app);
    delete_view::render_delete_note_view(f, app);
    confirm_view::render_confirm_view(f, app);
    app.notifications.render(f, f.area());
}
