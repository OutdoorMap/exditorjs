# EditorJS Converter

A high-performance Rust library for converting HTML and Markdown to Editor.js JSON format.

## Features

- ✅ HTML to Editor.js conversion
- ✅ Markdown to Editor.js conversion
- ✅ Support for multiple block types:
  - Paragraphs
  - Headings (H1-H6)
  - Lists (ordered and unordered)
  - Code blocks with language support
  - Blockquotes
  - Images
  - Tables
  - Horizontal rules
  - Raw HTML blocks
- ✅ Comprehensive error handling
- ✅ Easy to use API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
exditorjs_native = "0.1"
```

## Usage

### Convert HTML to Editor.js

```rust
use exditorjs_native::{html_to_editorjs, EditorJsDocument};

fn main() {
    let html = r#"
        <h1>Welcome</h1>
        <p>This is a paragraph.</p>
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
        </ul>
    "#;

    match html_to_editorjs(html) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);
            println!("{}", serde_json::to_string_pretty(&doc).unwrap());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Output:
```json
{
    "time": 1702080000000,
    "blocks": [
        {
            "id": "eLywdl3S5K",
            "type": "heading",
            "data": {
                "text": "Welcome",
                "level": 1
            }
        },
        {
            "id": "abc1234567",
            "type": "paragraph",
            "data": {
                "text": "This is a paragraph."
            }
        },
        {
            "id": "def1234567",
            "type": "list",
            "data": {
                "style": "unordered",
                "items": ["Item 1", "Item 2"]
            }
        }
    ],
    "version": "2.25.0"
}
```

### Convert Markdown to Editor.js

```rust
use exditorjs_native::{markdown_to_editorjs, EditorJsDocument};

fn main() {
    let markdown = r#"
# Welcome

This is a paragraph.

- Item 1
- Item 2

```rust
let x = 5;
```
    "#;

    match markdown_to_editorjs(markdown) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);
            println!("{}", serde_json::to_string_pretty(&doc).unwrap());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Output:
```json
{
    "time": 1702080000000,
    "blocks": [
        {
            "id": "eLywdl3S5K",
            "type": "heading",
            "data": {
                "text": "Welcome",
                "level": 1
            }
        },
        {
            "id": "abc1234567",
            "type": "paragraph",
            "data": {
                "text": "This is a paragraph."
            }
        },
        {
            "id": "def1234567",
            "type": "list",
            "data": {
                "style": "unordered",
                "items": ["Item 1", "Item 2"]
            }
        },
        {
            "id": "ghi1234567",
            "type": "code",
            "data": {
                "code": "let x = 5;",
                "language": "rust"
            }
        }
    ],
    "version": "2.25.0"
}
```

### Create a Full Editor.js Document

```rust
use exditorjs_native::{html_to_editorjs, EditorJsDocument};

fn main() {
    let html = "<h1>Title</h1><p>Content</p>";
    let blocks = html_to_editorjs(html).unwrap();
    let document = EditorJsDocument::new(blocks);
    
    println!("{}", serde_json::to_string_pretty(&document).unwrap());
}
```

## Supported Block Types

### Paragraph
```json
{
  "id": "uniqueId1",
  "type": "paragraph",
  "data": {
    "text": "Hello world"
  }
}
```

### Heading
```json
{
  "id": "uniqueId2",
  "type": "heading",
  "data": {
    "text": "Title",
    "level": 2
  }
}
```

### List
```json
{
  "id": "uniqueId3",
  "type": "list",
  "data": {
    "style": "unordered",
    "items": ["Item 1", "Item 2"]
  }
}
```

### Code
```json
{
  "id": "uniqueId4",
  "type": "code",
  "data": {
    "code": "let x = 5;",
    "language": "rust"
  }
}
```

### Quote
```json
{
  "id": "uniqueId5",
  "type": "quote",
  "data": {
    "text": "A wise quote",
    "caption": "Author",
    "alignment": "left"
  }
}
```

### Image
```json
{
  "id": "uniqueId6",
  "type": "image",
  "data": {
    "url": "https://example.com/image.jpg",
    "caption": "Image caption",
    "withBorder": false,
    "withBackground": false,
    "stretched": false
  }
}
```

### Table
```json
{
  "id": "uniqueId7",
  "type": "table",
  "data": {
    "content": [
      ["Header 1", "Header 2"],
      ["Cell 1", "Cell 2"]
    ]
  }
}
```

## HTML Support

Supported HTML tags:
- `<h1>` - `<h6>` - Headings
- `<p>` - Paragraphs
- `<ul>` - Unordered lists
- `<ol>` - Ordered lists
- `<li>` - List items
- `<code>`, `<pre>` - Code blocks
- `<blockquote>` - Quotes
- `<img>` - Images
- `<table>`, `<tr>`, `<td>`, `<th>` - Tables
- `<hr>` - Horizontal rule

## Markdown Support

Supported Markdown syntax:
- `# Heading` - Headings (levels 1-6)
- `- Item` - Unordered lists
- `1. Item` - Ordered lists
- `` ``` `` - Code blocks with optional language
- `> Quote` - Blockquotes
- `![alt](url)` - Images
- `| Header | Header |` - Tables
- `---` - Horizontal rules

## Error Handling

The library uses a custom `Result` type that returns `Error` on failure:

```rust
use exditorjs_native::{html_to_editorjs, Error};

match html_to_editorjs("<invalid") {
    Ok(blocks) => println!("Success"),
    Err(Error::HtmlParseError(msg)) => println!("Parse error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Performance

This library is optimized for performance:
- Uses regex for efficient HTML/Markdown parsing
- No external HTML parsers (lightweight)
- Zero-copy operations where possible
- Minimal allocations

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.