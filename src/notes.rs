use std::fs;
use std::io;
use std::path::Path;

use crate::helpers::slugify;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NoteState {
    Saved,
    New,
    Edited,
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note {
    pub name: String,
    pub text: Vec<String>,
    pub state: NoteState,
}

impl Note {
    pub fn new(name: String) -> Self {
        Note {
            name,
            text: Vec::new(),
            state: NoteState::Default,
        }
    }

    pub fn load(&mut self, vault: &Path) -> io::Result<()> {
        if !self.text.is_empty() {
            return Ok(());
        }
        let content = fs::read_to_string(vault.join(&self.name))?;

        self.text = content
            .split('\n')
            .map(|line| line.trim_end_matches('\r').to_string())
            .collect();

        self.state = NoteState::Saved;

        Ok(())
    }

    pub fn save_on_memory(&mut self, text: Vec<String>) {
        self.text = text;
        self.mark_as_edited();
    }

    pub fn create(vault: &Path, title: &str) -> io::Result<Self> {
        let filename = format!("{}.md", slugify(title));

        let mut note = Self {
            name: filename,
            text: vec![format!("# {}", title), String::new(), String::new()],
            state: NoteState::New,
        };

        note.save(vault)?;

        Ok(note)
    }

    pub fn delete(&self, vault: &Path) -> io::Result<()> {
        fs::remove_file(vault.join(&self.name))
    }

    pub fn mark_as_edited(&mut self) {
        if self.state == NoteState::Saved {
            self.state = NoteState::Edited;
        }
    }

    pub fn save(&mut self, vault: &Path) -> io::Result<()> {
        let path = vault.join(&self.name);
        let content = self.text.join("\n");

        fs::write(path, content)?;

        self.state = NoteState::Saved;

        Ok(())
    }
}
