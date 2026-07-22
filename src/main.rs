use std::fs;
use std::io;
use std::path::Path;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui_textarea::TextArea;
use ratatui_notifications::{Notification, Notifications, Level, Anchor, Animation, SizeConstraint, NotificationError};
use std::time::Duration;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

// --- Estado de la app ---
struct App {
    notes: Vec<String>,
    selected: usize, // índice de la nota seleccionada
    mode: Mode,      // nuevo campo
    text_area: TextArea<'static>,
    notifications: Notifications,
}

enum Mode {
    Normal,             // navegando la lista (como ahora)
    NewNote(String),    // escribiendo el título de una nota nueva
    DeleteNote(String), // borrando una nota
    EditNote(String )
}


impl App {
    fn new(vault: &Path) -> io::Result<Self> {
        let notes = list_notes(vault)?;
        Ok(App {
            notes,
            selected: 0,
            mode: Mode::Normal,
            text_area: TextArea::default(),
            notifications: Notifications::new()
        })
    }

    fn next(&mut self) {
        if !self.notes.is_empty() {
            self.selected = (self.selected + 1) % self.notes.len();
        }
    }

    fn previous(&mut self) {
        if !self.notes.is_empty() {
            self.selected = if self.selected == 0 {
                self.notes.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    fn add_notification(&mut self, kind: Level, title: String, text: String) {
        let notif = Notification::new(text)
            .title(title)
            .level(kind)
            .anchor(Anchor::BottomRight)
            .animation(Animation::Fade)
            .max_size(SizeConstraint::Absolute(30),SizeConstraint::Absolute(1),)
            .build().unwrap();

        self.notifications.add(notif).unwrap();
    }
}

fn list_notes(vault: &Path) -> io::Result<Vec<String>> {
    let mut notes = Vec::new();
    for entry in fs::read_dir(vault)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                notes.push(name.to_string());
            }
        }
    }
    notes.sort();
    Ok(notes)
}

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
            // Divide la pantalla en 2 columnas: 30% lista, 70% preview
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
                                let note_text = read_note_content(Path::new("vault"), &note.clone());
                                let  lines: Vec<String> = note_text.split('\n').map(String::from).collect();
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
                            app.add_notification(Level::Warn, "Exito".to_string(), "Archivo creado".to_string());
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
                            app.add_notification(Level::Error, "Exito".to_string(), "Archivo eliminado".to_string());
                            app.mode = Mode::Normal; // cancelar
                        }
                        _ => {}
                    },
                    Mode::EditNote(note) => match key.code {
                        KeyCode::Esc => {
                            app.mode = Mode::Normal; // cancelar
                        },
                        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let text: String = app.text_area.lines().join("\n");
                            let path = Path::new("vault").join(note);
                            save_note(&path, &text)?;
                            app.add_notification(Level::Info, "Exito".to_string(), "Archivo guardado".to_string());

                            app.mode = Mode::Normal; // cancelar
                        },
                        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.text_area.insert_newline();
                            app.text_area.insert_str("[ ] ");
                        }
                        _ => {
                            app.text_area.input(key);
                        }
                    }
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

fn read_note_content(vault: &Path, filename: &str) -> String {
    fs::read_to_string(vault.join(filename)).unwrap_or_else(|_| "(no se pudo leer)".to_string())
}

fn create_note(vault: &Path, title: &str) -> io::Result<String> {
    let filename = format!("{}.md", slugify(title));
    let path = vault.join(&filename);
    fs::write(&path, format!("# {}\n\n", title))?;
    Ok(filename)
}

fn slugify(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect()
}

fn delete_note(file_path: &Path) -> Result<(), io::Error> {
    fs::remove_file(&file_path)?;
    Ok(())
}

fn save_note(file_path: &Path, text: &str) -> io::Result<()>{
    fs::write(&file_path, text)?;
    Ok(())

}

