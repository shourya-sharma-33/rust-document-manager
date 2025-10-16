use std::fs;
use std::io;
use std::path::Path;

trait DocumentElement {
    fn render(&self) -> String;
}
struct TextElement {
    text: String,
}

impl TextElement {
    fn new<S: Into<String>>(s: S) -> Self {
        Self { text: s.into() }
    }
}

impl DocumentElement for TextElement {
    fn render(&self) -> String {
        self.text.clone()
    }
}
struct ImageElement {
    path: String,
}

impl ImageElement {
    fn new<S: Into<String>>(p: S) -> Self {
        Self { path: p.into() }
    }
}

impl DocumentElement for ImageElement {
    fn render(&self) -> String {
        format!("[Image: {}]", self.path)
    }
}
struct NewLineElement;

impl NewLineElement {
    fn new() -> Self {
        Self
    }
}

impl DocumentElement for NewLineElement {
    fn render(&self) -> String {
        "\n".to_string()
    }
}
struct TabSpaceElement;

impl TabSpaceElement {
    fn new() -> Self {
        Self
    }
}

impl DocumentElement for TabSpaceElement {
    fn render(&self) -> String {
        "\t".to_string()
    }
}

struct Document {
    elements: Vec<Box<dyn DocumentElement>>,
    cached_render: Option<String>,
}

impl Document {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            cached_render: None,
        }
    }

    fn add_element(&mut self, elem: Box<dyn DocumentElement>) {
        self.elements.push(elem);
        self.cached_render = None;
    }

    fn render(&mut self) -> String {
        if let Some(ref cached) = self.cached_render {
            return cached.clone();
        }

        let mut result = String::new();
        for e in &self.elements {
            result.push_str(&e.render());
        }
        self.cached_render = Some(result.clone());
        result
    }
}

trait Persistence {
    fn save(&self, data: &str) -> io::Result<()>;
}

struct FileStorage {
    filename: String,
}

impl FileStorage {
    fn new<S: Into<String>>(filename: S) -> Self {
        Self {
            filename: filename.into(),
        }
    }
}

impl Persistence for FileStorage {
    fn save(&self, data: &str) -> io::Result<()> {
        fs::write(&self.filename, data)
    }
}
struct DbStorage;

impl DbStorage {
    fn new() -> Self {
        Self
    }
}

impl Persistence for DbStorage {
    fn save(&self, _data: &str) -> io::Result<()> {
        println!("(DbStorage) Pretend saving to DB...");
        Ok(())
    }
}
struct DocumentEditor {
    document: Document,
    storage: Box<dyn Persistence>,
}

impl DocumentEditor {
    fn new(document: Document, storage: Box<dyn Persistence>) -> Self {
        Self { document, storage }
    }

    fn add_text<S: Into<String>>(&mut self, text: S) {
        self.document.add_element(Box::new(TextElement::new(text)));
    }

    fn add_image<S: Into<String>>(&mut self, path: S) {
        self.document
            .add_element(Box::new(ImageElement::new(path)));
    }

    fn add_newline(&mut self) {
        self.document
            .add_element(Box::new(NewLineElement::new()));
    }

    fn add_tab(&mut self) {
        self.document
            .add_element(Box::new(TabSpaceElement::new()));
    }

    fn render_document(&mut self) -> String {
        self.document.render()
    }

    fn save_document(&self) -> io::Result<()> {
        let mut result = String::new();
        for e in &self.document.elements {
            result.push_str(&e.render());
        }
        self.storage.save(&result)
    }
}


fn main() -> io::Result<()> {
    let mut doc = Document::new();
    let storage = Box::new(FileStorage::new("document.txt"));

    let mut editor = DocumentEditor::new(doc, storage);

    editor.add_text("Hello, world!");
    editor.add_newline();
    editor.add_text("This is a real-world document editor example.");
    editor.add_newline();
    editor.add_tab();
    editor.add_text("Indented text after a tab space.");
    editor.add_newline();
    editor.add_image("picture.jpg");

    let output = editor.render_document();
    println!("{}", output);

    match editor.save_document() {
        Ok(_) => println!("Document saved to document.txt"),
        Err(e) => eprintln!("Error saving document: {}", e),
    }

    Ok(())
}
