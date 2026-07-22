mod app;
mod mode;
mod notes;
mod ui;

use crate::app::App;
use crate::mode::Mode;

use std::io;
use std::path::Path;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Color, Style},
};
use ratatui_notifications::Level;
use ratatui_textarea::TextArea;
use std::time::Duration;

use crate::notes::{create_note, delete_note, list_notes, read_note_content, save_note};

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
            ui::draw(f, &mut app, Path::new("vault"));
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match &mut app.mode {
                        Mode::Normal => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('j') | KeyCode::Down => app.next(),
                            KeyCode::Char('k') | KeyCode::Up => app.previous(),
                            KeyCode::Char('n') => {
                                app.mode = Mode::NewNote(String::new()); // entra a modo creación
                            }
                            KeyCode::Char('e') => {
                                if let Some(note) = app.notes.get(app.selected) {
                                    let note_text =
                                        read_note_content(Path::new("vault"), &note.clone());
                                    let lines: Vec<String> =
                                        note_text.split('\n').map(String::from).collect();
                                    app.text_area = TextArea::new(lines);
                                    let style = Style::default().fg(Color::DarkGray);
                                    app.text_area.set_line_number_style(style);

                                    app.mode = Mode::EditNote(note.clone());
                                }
                            }

                            KeyCode::Char('d') => {
                                if let Some(note) = app.notes.get(app.selected) {
                                    app.mode = Mode::DeleteNote(note.clone());
                                }
                            }
                            _ => {}
                        },
                        Mode::NewNote(input) => match key.code {
                            KeyCode::Esc => {
                                app.mode = Mode::Normal; // cancelar
                            }
                            KeyCode::Enter => {
                                let title = input.clone();
                                let filename = create_note(Path::new("vault"), &title)?;
                                app.notes = list_notes(Path::new("vault"))?; // recarga la lista
                                app.selected =
                                    app.notes.iter().position(|n| n == &filename).unwrap_or(0);
                                app.add_notification(
                                    Level::Warn,
                                    "Exito".to_string(),
                                    "Archivo creado".to_string(),
                                );
                                app.mode = Mode::Normal;
                            }
                            KeyCode::Char(c) => {
                                input.push(c); // agrega el caracter al título
                            }
                            KeyCode::Backspace => {
                                input.pop(); // borra el último caracter
                            }
                            _ => {}
                        },
                        Mode::DeleteNote(note) => match key.code {
                            KeyCode::Esc => {
                                app.mode = Mode::Normal; // cancelar
                            }
                            KeyCode::Char('d') => {
                                let path = Path::new("vault").join(note);
                                delete_note(&path)?;
                                app.notes = list_notes(Path::new("vault"))?;
                                if app.notes.len() == 0 {
                                    app.selected = 0;
                                } else if app.selected >= app.notes.len() {
                                    app.selected = app.notes.len() - 1;
                                }
                                app.add_notification(
                                    Level::Error,
                                    "Exito".to_string(),
                                    "Archivo eliminado".to_string(),
                                );
                                app.mode = Mode::Normal; // cancelar
                            }
                            _ => {}
                        },
                        Mode::EditNote(note) => match key.code {
                            KeyCode::Esc => {
                                app.mode = Mode::Normal; // cancelar
                            }
                            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                let text: String = app.text_area.lines().join("\n");
                                let path = Path::new("vault").join(note);
                                save_note(&path, &text)?;
                                app.add_notification(
                                    Level::Info,
                                    "Exito".to_string(),
                                    "Archivo guardado".to_string(),
                                );

                                app.mode = Mode::Normal; // cancelar
                            }
                            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                app.text_area.insert_newline();
                                app.text_area.insert_str("[ ] ");
                            }
                            _ => {
                                app.text_area.input(key);
                            }
                        },
                    }
                }
            }
        }
    }

    // --- Restaurar la terminal a su estado normal ---
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
