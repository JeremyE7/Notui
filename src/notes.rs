use std::fs;
use std::io;
use std::path::Path;

pub fn read_note_content(vault: &Path, filename: &str) -> String {
    fs::read_to_string(vault.join(filename)).unwrap_or_else(|_| "(no se pudo leer)".to_string())
}

pub fn create_note(vault: &Path, title: &str) -> io::Result<String> {
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

pub fn delete_note(file_path: &Path) -> Result<(), io::Error> {
    fs::remove_file(&file_path)?;
    Ok(())
}

pub fn save_note(file_path: &Path, text: &str) -> io::Result<()> {
    fs::write(&file_path, text)?;
    Ok(())
}

pub fn list_notes(vault: &Path) -> io::Result<Vec<String>> {
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
