pub struct Note {
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(title: &str) -> Note {
        Note {
            title: title.to_string(),
            content: String::new(),
        }
    }

    pub fn append_content(&mut self, text: &str) {
        self.content += text;
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn display(&self) {
        println!("Title: {}", self.title);
        println!("{}", self.content);
    }
}
