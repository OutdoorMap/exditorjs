use exditorjs_native::{html_to_editorjs, markdown_to_editorjs, EditorJsDocument};

fn main() {
    println!("=== EditorJS Converter Examples ===\n");

    // Example 1: Simple HTML conversion
    println!("Example 1: HTML to Editor.js Document");
    let html = r#"
        <h1>Welcome to EditorJS</h1>
        <p>This is a simple paragraph with some content.</p>
        <ul>
            <li>First item</li>
            <li>Second item</li>
            <li>Third item</li>
        </ul>
    "#;

    match html_to_editorjs(html) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);
            println!("{}\n", serde_json::to_string_pretty(&doc).unwrap());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 2: Markdown conversion
    println!("Example 2: Markdown to Editor.js Document");
    let markdown = r#"
# Getting Started

This is a **markdown** document with various elements.

## Features

- Easy to use
- Powerful conversion
- Well documented

## Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```

> This is a blockquote with some wisdom.

---

Thank you for using EditorJS Converter!
    "#;

    match markdown_to_editorjs(markdown) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);
            println!("{}\n", serde_json::to_string_pretty(&doc).unwrap());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 3: Complex HTML
    println!("Example 3: Complex HTML with mixed content");
    let complex_html = r#"
        <h2>About Us</h2>
        <p>We provide the best conversion tools.</p>
        <blockquote>Quality is our priority</blockquote>
        <img src="https://example.com/image.jpg" alt="Example Image">
        <code>let result = convert(input);</code>
    "#;

    match html_to_editorjs(complex_html) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);
            println!("{}\n", serde_json::to_string_pretty(&doc).unwrap());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
