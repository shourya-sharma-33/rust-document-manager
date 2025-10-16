
# 📝 Rust Document Editor

A modular and extensible **Document Editor** written in **Rust**, demonstrating core OOP design patterns (traits, polymorphism, caching, and persistence abstraction).  
It supports adding **text**, **images**, **newlines**, and **tab spaces** to a document, rendering it, and saving it to various storage backends such as **file storage** or a simulated **database**.

---

## 📖 Features

- ✍️ Add text elements dynamically  
- 🖼️ Add image placeholders  
- ↩️ Add newlines and tab spaces  
- 💾 Save to a file (via `FileStorage`)  
- 🗄️ Simulate saving to a database (via `DbStorage`)  
- ⚡ Render caching to avoid redundant re-computation  

---

## 🧩 Code Structure

| Component | Description |
|------------|-------------|
| **DocumentElement** | A trait defining the `render()` method for all document elements. |
| **TextElement / ImageElement / NewLineElement / TabSpaceElement** | Concrete types implementing the `DocumentElement` trait. |
| **Document** | Holds a list of elements and caches its rendered result. |
| **Persistence Trait** | Abstract layer for saving data to different backends. |
| **FileStorage / DbStorage** | Implementations of the `Persistence` trait. |
| **DocumentEditor** | High-level interface for building and saving documents. |

---

## 🧠 Example Output


```

Hello, world!  
This is a real-world document editor example.  
Indented text after a tab space.  
[Image: picture.jpg]

```

---

## 🚀 Running the Project

```bash
# Initialize a new Rust project
cargo new document_editor
cd document_editor

# Replace src/main.rs with this code
# Then run:
cargo run

```

This will:

-   Print the rendered document to the console.
    
-   Save it to `document.txt` in your project directory.
    

----------

## 🗂️ UML Diagram

Below is a UML class diagram showing the structure and relationships between components:

![UML Diagram](/uml2.svg)

----------

## 🧩 Possible Extensions

-   Add formatting elements (bold, italic, underline).
    
-   Add markdown or HTML export.
    
-   Implement real database persistence (e.g., SQLite or PostgreSQL).
    
-   Add unit tests using Rust’s built-in `#[test]` framework.
    

----------

## 🦀 Built With

-   **Rust** 🦀
    
-   **Standard Library (`std::fs`, `std::io`, `std::path`)**
    

----------