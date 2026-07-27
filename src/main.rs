mod app;
mod editor;
mod events;
mod mode;
mod notes;
mod ui;

use crate::app::App;

use std::io;
use std::path::Path;

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::Duration;

fn main() -> io::Result<()> {
    // --- Setup de la terminal (modo "raw" = capturamos cada tecla directo) ---
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(Path::new("vault"))?;
    // --- Loop principal ---
    loop {
        app.notifications.tick(Duration::from_millis(200));
        terminal.draw(|f| {
            ui::draw(f, &mut app);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Release && events::handle_key(&mut app, key)? {
                    break;
                }
            }
        }
    }

    // --- Restaurar la terminal a su estado normal ---
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
