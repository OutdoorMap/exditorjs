#[cfg(test)]
mod tests {
    use exditorjs_native::{html_to_editorjs, markdown_to_editorjs, EditorJsDocument};

    #[test]
    fn test_html_paragraph() {
        let html = "<p>Hello World</p>";
        let result = html_to_editorjs(html);
        assert!(result.is_ok());
        let blocks = result.unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_html_heading() {
        let html = "<h1>Title</h1>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_html_multiple_headings() {
        let html = "<h1>H1</h1><h2>H2</h2><h3>H3</h3>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn test_html_list() {
        let html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_html_ordered_list() {
        let html = "<ol><li>First</li><li>Second</li></ol>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_html_blockquote() {
        let html = "<blockquote>A wise quote</blockquote>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_html_code() {
        let html = "<code>let x = 5;</code>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_html_image() {
        let html = r#"<img src="https://example.com/image.jpg" alt="Test Image">"#;
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_html_mixed_content() {
        let html = r#"<h1>Title</h1><p>Paragraph</p><ul><li>Item</li></ul>"#;
        let blocks = html_to_editorjs(html).unwrap();
        assert!(blocks.len() > 0);
    }

    #[test]
    fn test_markdown_heading() {
        let md = "# Hello";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_multiple_headings() {
        let md = "# H1\n## H2\n### H3";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn test_markdown_paragraph() {
        let md = "This is a paragraph.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_unordered_list() {
        let md = "- Item 1\n- Item 2\n- Item 3";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_ordered_list() {
        let md = "1. First\n2. Second\n3. Third";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_code_block() {
        let md = "```rust\nlet x = 5;\n```";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_blockquote() {
        let md = "> A wise quote";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_image() {
        let md = r#"![Alt text](https://example.com/image.jpg)"#;
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_horizontal_rule() {
        let md = "Above\n\n---\n\nBelow";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert!(blocks.len() > 0);
    }

    #[test]
    fn test_markdown_mixed_content() {
        let md = r#"
# Title

This is a paragraph.

- Item 1
- Item 2

```javascript
console.log("hello");
```
        "#;
        let blocks = markdown_to_editorjs(md).unwrap();
        assert!(blocks.len() > 0);
    }

    #[test]
    fn test_editorjs_document_creation() {
        let html = "<p>Test</p>";
        let blocks = html_to_editorjs(html).unwrap();
        let doc = EditorJsDocument::new(blocks);
        assert_eq!(doc.version, "2.25.0");
        assert!(doc.time > 0);
        assert_eq!(doc.blocks.len(), 1);
        // Check that block has an ID
        assert!(!doc.blocks[0].id.is_empty());
    }

    #[test]
    fn test_html_entities_decoding() {
        let html = "<p>&lt;html&gt; &amp; &quot;quoted&quot;</p>";
        let blocks = html_to_editorjs(html).unwrap();
        assert!(blocks.len() > 0);
    }

    #[test]
    fn test_markdown_table() {
        let md = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1 | Cell 2 |";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert!(blocks.len() > 0);
    }
}
