use std::fs;
use crate::note::Note;

pub struct Editor {
    notes: Vec<Note>,
    selected_note_index: Option<usize>,
    undo_buffer: Vec<String>, 
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            notes: Vec::new(),
            selected_note_index: None,
            undo_buffer: Vec::new(), 
        }
    }

    pub fn create_new_note(&mut self, title: &str) {
        let new_note = Note::new(title);
        self.notes.push(new_note);
        self.select_note_by_title(title);
    }

    pub fn insert_text_to_selected(&mut self, text: &str) {
        if let Some(index) = self.selected_note_index {
            self.notes[index].append_content(text);

            self.undo_buffer.push(self.notes[index].get_content().to_string());
        }
    }

    pub fn display_selected_note(&self) {
        if let Some(index) = self.selected_note_index {
            self.notes[index].display();
        }
    }

  pub fn select_note_by_title(&mut self, title: &str) -> bool {
      for (index, note) in self.notes.iter().enumerate() {
          if note.title == title {
              self.selected_note_index = Some(index);
              return true; 
          }
      }
      false 
  }

  pub fn load_note_from_file(&mut self, filename: &str) -> bool {
      if let Ok(content) = fs::read_to_string(filename) {
          let title = filename.trim_end_matches(".txt");
          self.create_new_note(title);
          if let Some(index) = self.selected_note_index {
              self.notes[index].append_content(&content);
              return true; 
          }
      }
      false 
  }


    pub fn save_selected_note_to_file(&self) {
        if let Some(index) = self.selected_note_index {
            if let Err(_) = fs::write(format!("notes/{}.txt", self.notes[index].title), self.notes[index].get_content()) {
                eprintln!("Error saving note to file!");
            }
        }
    }



    pub fn undo_last_edit(&mut self) {
        if let Some(index) = self.selected_note_index {
            if let Some(undo_content) = self.undo_buffer.pop() {
                self.notes[index].content = undo_content;
            }
        }
    }
}
