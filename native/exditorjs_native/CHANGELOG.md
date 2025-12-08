# Changelog

All notable changes to the EditorJS Converter project will be documented in this file.

## [0.1.0] - 2025-12-08

### Added

#### Core Features
- **HTML to Editor.js conversion**: Convert HTML documents to Editor.js JSON format
- **Markdown to Editor.js conversion**: Convert Markdown documents to Editor.js JSON format
- **EditorJsDocument**: Wrapper type for complete Editor.js documents with metadata

#### Supported Block Types
- **Paragraph**: Basic text content blocks
- **Heading**: Heading blocks with levels 1-6
- **List**: Both ordered and unordered lists
- **Code**: Code blocks with optional language specification
- **Quote**: Blockquote blocks
- **Image**: Image blocks with optional caption and styling options
- **Table**: Table blocks with cell content
- **Delimiter**: Horizontal rule blocks
- **Raw**: Raw HTML blocks for unsupported content

#### HTML Support
- `<h1>` - `<h6>` heading tags
- `<p>` paragraph tags
- `<ul>`, `<ol>`, `<li>` list tags
- `<code>`, `<pre>` code tags
- `<blockquote>` quote tags
- `<img>` image tags with src and alt attributes
- `<table>`, `<tr>`, `<td>`, `<th>` table elements
- `<hr>` horizontal rule tag
- HTML entity decoding (&lt;, &gt;, &amp;, &quot;, &#39;, &nbsp;, &copy;, &reg;)

#### Markdown Support
- ATX-style headings (#, ##, ###, etc.)
- Unordered lists (-, *)
- Ordered lists (1., 2., etc.)
- Code blocks with language specification (```language)
- Blockquotes (>)
- Images (![alt](url))
- Tables (with pipe separators)
- Horizontal rules (---, ***, ___)

#### Error Handling
- Custom `Error` enum with specific error variants
- `Result<T>` type alias for ergonomic error handling
- Error types: HtmlParseError, MarkdownParseError, InvalidInput, SerializationError

#### Project Structure
- Modular architecture with separate modules for HTML and Markdown parsing
- Clean separation of concerns (models, error handling, block parsing)
- Comprehensive documentation and examples

#### Testing
- Unit tests in HTML and Markdown modules
- Integration tests covering various conversion scenarios
- 20+ test cases for different content types

#### Documentation
- Detailed README with usage examples
- Inline code documentation
- Example program demonstrating library usage
- Clear error message formatting

### Technical Details
- **Language**: Rust 2021 edition
- **Dependencies**: 
  - `serde` & `serde_json` for JSON serialization
  - `regex` for HTML/Markdown parsing
  - `thiserror` for error handling
- **Performance**: Optimized regex-based parsing without heavyweight HTML parsers
- **Safety**: Memory-safe implementation with no unsafe code

### Project Files
- `Cargo.toml`: Project manifest with dependencies
- `src/lib.rs`: Library root and public API
- `src/error.rs`: Error types and handling
- `src/models.rs`: Editor.js block data models
- `src/html.rs`: HTML to Editor.js conversion
- `src/markdown.rs`: Markdown to Editor.js conversion
- `src/blocks/`: Block parsing utilities module
- `examples/basic.rs`: Comprehensive usage examples
- `tests/integration_tests.rs`: Integration test suite
- `README.md`: Complete documentation

## Future Enhancements

- [ ] Advanced HTML parsing with html5ever crate
- [ ] Nested list support
- [ ] Inline formatting preservation (bold, italic, underline)
- [ ] Link block type support
- [ ] Embed block type enhancement
- [ ] Custom block type handling
- [ ] Performance benchmarks
- [ ] WASM bindings
- [ ] CLI tool for batch conversions
- [ ] Configuration options for parsing behavior
