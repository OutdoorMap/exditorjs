use crate::error::{Error, Result};
use crate::models::*;
use regex::Regex;

/// Convert Markdown to Editor.js blocks
pub fn markdown_to_editorjs(markdown: &str) -> Result<Vec<EditorJsBlock>> {
    let parser = MarkdownParser::new(markdown);
    parser.parse()
}

struct MarkdownParser {
    markdown: String,
    lines: Vec<String>,
}

impl MarkdownParser {
    fn new(markdown: &str) -> Self {
        let lines: Vec<String> = markdown.lines().map(|s| s.to_string()).collect();
        MarkdownParser {
            markdown: markdown.to_string(),
            lines,
        }
    }

    fn parse(&self) -> Result<Vec<EditorJsBlock>> {
        let mut blocks = Vec::new();
        let mut i = 0;

        while i < self.lines.len() {
            let line = &self.lines[i];

            if line.is_empty() {
                i += 1;
                continue;
            }

            // Check for headings
            if let Some(level) = self.parse_heading_level(line) {
                let text = self.extract_heading_text(line, level);
                blocks.push(EditorJsBlock::Heading {
                    data: HeadingData { text, level },
                });
                i += 1;
                continue;
            }

            // Check for code blocks
            if line.starts_with("```") {
                let (code_block, next_i) = self.parse_code_block(i);
                if let Some(block) = code_block {
                    blocks.push(block);
                }
                i = next_i;
                continue;
            }

            // Check for lists
            if line.trim_start().starts_with("- ") || line.trim_start().starts_with("* ") {
                let (list_block, next_i) = self.parse_unordered_list(i);
                blocks.push(list_block);
                i = next_i;
                continue;
            }

            if line.trim_start().chars().next().map_or(false, |c| c.is_numeric())
                && line.contains(". ")
            {
                let (list_block, next_i) = self.parse_ordered_list(i);
                blocks.push(list_block);
                i = next_i;
                continue;
            }

            // Check for blockquotes
            if line.trim_start().starts_with("> ") {
                let (quote_block, next_i) = self.parse_blockquote(i);
                blocks.push(quote_block);
                i = next_i;
                continue;
            }

            // Check for horizontal rule
            if self.is_horizontal_rule(line) {
                blocks.push(EditorJsBlock::Delimiter {});
                i += 1;
                continue;
            }

            // Check for images
            if let Some(image_block) = self.parse_image_markdown(line) {
                blocks.push(image_block);
                i += 1;
                continue;
            }

            // Check for tables
            if i + 1 < self.lines.len() && self.is_table_separator(&self.lines[i + 1]) {
                let (table_block, next_i) = self.parse_table(i);
                blocks.push(table_block);
                i = next_i;
                continue;
            }

            // Treat as paragraph
            let (paragraph, next_i) = self.parse_paragraph(i);
            blocks.push(EditorJsBlock::Paragraph {
                data: ParagraphData { text: paragraph },
            });
            i = next_i;
        }

        Ok(blocks)
    }

    fn parse_heading_level(&self, line: &str) -> Option<u8> {
        let trimmed = line.trim_start();
        for i in 1..=6 {
            if trimmed.starts_with(&format!("{} ", "#".repeat(i))) {
                return Some(i as u8);
            }
        }
        None
    }

    fn extract_heading_text(&self, line: &str, level: u8) -> String {
        let prefix = format!("{} ", "#".repeat(level as usize));
        let mut text = line.trim_start()
            .strip_prefix(&prefix)
            .unwrap_or(line)
            .trim()
            .to_string();
        
        // Convert inline formatting
        text = self.convert_inline_formatting(&text);
        // Convert markdown links to HTML links
        text = self.convert_markdown_links(&text);
        text
    }

    fn parse_code_block(&self, start: usize) -> (Option<EditorJsBlock>, usize) {
        let mut code_lines = Vec::new();
        let mut language = String::new();
        let mut i = start + 1;

        // Get language if specified
        let first_line = &self.lines[start];
        if first_line.len() > 3 {
            language = first_line[3..].trim().to_string();
        }

        // Collect code lines
        while i < self.lines.len() && !self.lines[i].trim().starts_with("```") {
            code_lines.push(self.lines[i].clone());
            i += 1;
        }

        let block = EditorJsBlock::Code {
            data: CodeData {
                code: code_lines.join("\n"),
                language: if language.is_empty() {
                    None
                } else {
                    Some(language)
                },
            },
        };

        (Some(block), if i < self.lines.len() { i + 1 } else { i })
    }

    fn parse_unordered_list(&self, start: usize) -> (EditorJsBlock, usize) {
        let mut items = Vec::new();
        let mut i = start;

        while i < self.lines.len() {
            let line = &self.lines[i];
            let trimmed = line.trim_start();

            if !trimmed.starts_with("- ") && !trimmed.starts_with("* ") {
                break;
            }

            let mut item = trimmed[2..].trim().to_string();
            // Convert inline formatting
            item = self.convert_inline_formatting(&item);
            // Convert markdown links to HTML links
            item = self.convert_markdown_links(&item);
            items.push(item);
            i += 1;
        }

        (
            EditorJsBlock::List {
                data: ListData {
                    style: "unordered".to_string(),
                    items,
                },
            },
            i,
        )
    }

    fn parse_ordered_list(&self, start: usize) -> (EditorJsBlock, usize) {
        let mut items = Vec::new();
        let mut i = start;

        while i < self.lines.len() {
            let line = &self.lines[i];
            let trimmed = line.trim_start();

            if let Some(dot_pos) = trimmed.find(". ") {
                if trimmed[..dot_pos].chars().all(|c| c.is_numeric()) {
                    let mut item = trimmed[dot_pos + 2..].trim().to_string();
                    // Convert inline formatting
                    item = self.convert_inline_formatting(&item);
                    // Convert markdown links to HTML links
                    item = self.convert_markdown_links(&item);
                    items.push(item);
                    i += 1;
                    continue;
                }
            }
            break;
        }

        (
            EditorJsBlock::List {
                data: ListData {
                    style: "ordered".to_string(),
                    items,
                },
            },
            i,
        )
    }

    fn parse_blockquote(&self, start: usize) -> (EditorJsBlock, usize) {
        let mut lines = Vec::new();
        let mut i = start;

        while i < self.lines.len() && self.lines[i].trim_start().starts_with("> ") {
            let line = self.lines[i].trim_start();
            lines.push(line[2..].trim().to_string());
            i += 1;
        }

        let mut text = lines.join(" ");
        // Convert inline formatting
        text = self.convert_inline_formatting(&text);
        // Convert markdown links to HTML links
        text = self.convert_markdown_links(&text);

        (
            EditorJsBlock::Quote {
                data: QuoteData {
                    text,
                    caption: None,
                    alignment: "left".to_string(),
                },
            },
            i,
        )
    }

    fn parse_image_markdown(&self, line: &str) -> Option<EditorJsBlock> {
        // Pattern: ![alt text](url 'title') or ![alt text](url "title") or ![alt text](url)
        let re = Regex::new(r#"!\[([^\]]*)\]\(([^)\s]+)(?:\s+['\"]([^'\"]*)['\"])?\)"#).unwrap();
        if let Some(cap) = re.captures(line) {
            let alt_text = cap.get(1).map(|m| m.as_str().to_string());
            let url = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
            let title = cap.get(3).map(|m| m.as_str().to_string());

            // Use title if available, otherwise use alt text as caption
            let caption = title.or(alt_text);

            return Some(EditorJsBlock::Image {
                data: ImageData {
                    url,
                    caption: if caption.as_ref().map_or(true, |c| c.is_empty()) {
                        None
                    } else {
                        caption
                    },
                    withBorder: None,
                    withBackground: None,
                    stretched: None,
                },
            });
        }
        None
    }

    fn parse_table(&self, start: usize) -> (EditorJsBlock, usize) {
        let mut content = Vec::new();
        let mut i = start;

        // Parse header
        if i < self.lines.len() {
            let header = self.parse_table_row(&self.lines[i]);
            content.push(header);
            i += 1;
        }

        // Skip separator
        if i < self.lines.len() {
            i += 1;
        }

        // Parse remaining rows
        while i < self.lines.len() && self.is_table_row(&self.lines[i]) {
            let row = self.parse_table_row(&self.lines[i]);
            content.push(row);
            i += 1;
        }

        (
            EditorJsBlock::Table {
                data: TableData { content },
            },
            i,
        )
    }

    fn parse_table_row(&self, line: &str) -> Vec<String> {
        line.split('|')
            .map(|cell| cell.trim().to_string())
            .filter(|cell| !cell.is_empty())
            .collect()
    }

    fn is_table_separator(&self, line: &str) -> bool {
        let cells: Vec<&str> = line.split('|').collect();
        if cells.len() < 2 {
            return false;
        }

        cells
            .iter()
            .filter(|cell| !cell.trim().is_empty())
            .all(|cell| {
                let trimmed = cell.trim();
                trimmed.chars().all(|c| c == '-' || c == ':' || c == ' ')
                    && trimmed.contains('-')
            })
    }

    fn is_table_row(&self, line: &str) -> bool {
        line.contains('|') && !self.is_table_separator(line)
    }

    fn parse_paragraph(&self, start: usize) -> (String, usize) {
        let mut lines = Vec::new();
        let mut i = start;

        while i < self.lines.len() && !self.lines[i].is_empty() {
            lines.push(self.lines[i].clone());
            i += 1;
        }

        // Skip empty lines
        while i < self.lines.len() && self.lines[i].is_empty() {
            i += 1;
        }

        let mut text = lines.join(" ");
        // Convert markdown inline formatting (bold, italic, strikethrough)
        text = self.convert_inline_formatting(&text);
        // Convert markdown links to HTML links with target="_blank"
        text = self.convert_markdown_links(&text);
        
        (text, i)
    }

    fn convert_inline_formatting(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Convert strikethrough: ~~text~~ -> <s>text</s>
        let strikethrough = Regex::new(r"~~([^~]+)~~").unwrap();
        result = strikethrough
            .replace_all(&result, "<s>$1</s>")
            .into_owned();
        
        // Convert bold: **text** -> <b>text</b>
        let bold = Regex::new(r"\*\*([^\*]+)\*\*").unwrap();
        result = bold
            .replace_all(&result, "<b>$1</b>")
            .into_owned();
        
        // Convert italic: _text_ -> <i>text</i> (but not in links or bold)
        let italic = Regex::new(r"_([^_]+)_").unwrap();
        result = italic
            .replace_all(&result, "<i>$1</i>")
            .into_owned();
        
        result
    }

    fn convert_markdown_links(&self, text: &str) -> String {
        // Pattern: [link text](url)
        let link_pattern = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        link_pattern
            .replace_all(text, |caps: &regex::Captures| {
                let link_text = &caps[1];
                let url = &caps[2];
                format!(r#"<a href="{}" target="_blank">{}</a>"#, url, link_text)
            })
            .into_owned()
    }

    fn is_horizontal_rule(&self, line: &str) -> bool {
        let trimmed = line.trim();
        (trimmed == "---" || trimmed == "***" || trimmed == "___")
            && trimmed.len() >= 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let md = "# Hello";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_parse_list() {
        let md = "- Item 1\n- Item 2";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_parse_code() {
        let md = "```rust\nlet x = 5;\n```";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_markdown_link_in_paragraph() {
        let md = "This is a [link to Google](https://google.com) in a paragraph.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains(r#"<a href="https://google.com" target="_blank">link to Google</a>"#));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_markdown_multiple_links() {
        let md = "Check [GitHub](https://github.com) and [Stack Overflow](https://stackoverflow.com).";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains(r#"<a href="https://github.com" target="_blank">GitHub</a>"#));
            assert!(data.text.contains(r#"<a href="https://stackoverflow.com" target="_blank">Stack Overflow</a>"#));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_markdown_link_in_heading() {
        let md = "# Visit [our site](https://example.com)";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Heading { data } = &blocks[0] {
            assert!(data.text.contains(r#"<a href="https://example.com" target="_blank">our site</a>"#));
            assert_eq!(data.level, 1);
        } else {
            panic!("Expected heading block");
        }
    }

    #[test]
    fn test_markdown_link_in_list() {
        let md = "- Learn [Rust](https://www.rust-lang.org/)\n- Read [The Book](https://doc.rust-lang.org/book/)";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.items.len(), 2);
            assert!(data.items[0].contains(r#"<a href="https://www.rust-lang.org/" target="_blank">Rust</a>"#));
            assert!(data.items[1].contains(r#"<a href="https://doc.rust-lang.org/book/" target="_blank">The Book</a>"#));
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_markdown_link_in_quote() {
        let md = "> Check [our blog](https://blog.example.com) for updates.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Quote { data } = &blocks[0] {
            assert!(data.text.contains(r#"<a href="https://blog.example.com" target="_blank">our blog</a>"#));
        } else {
            panic!("Expected quote block");
        }
    }

    #[test]
    fn test_markdown_link_with_special_chars() {
        let md = "Visit [API docs](https://example.com/api?param=value&other=123)";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains(r#"https://example.com/api?param=value&other=123"#));
            assert!(data.text.contains(r#"target="_blank""#));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_markdown_link_with_hash() {
        let md = "Jump to [section](https://example.com#section1)";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains(r#"https://example.com#section1"#));
        } else {
            panic!("Expected paragraph block");
        }
    }

    // Inline Formatting Tests
    #[test]
    fn test_inline_bold_in_paragraph() {
        let md = "This is **bold text** in a paragraph.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains("<b>bold text</b>"));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_inline_italic_in_paragraph() {
        let md = "This is _italic text_ in a paragraph.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains("<i>italic text</i>"));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_inline_strikethrough_in_paragraph() {
        let md = "This is ~~strikethrough text~~ in a paragraph.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains("<s>strikethrough text</s>"));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_inline_multiple_formats() {
        let md = "This is **bold**, _italic_, and ~~strikethrough~~ together.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            assert!(data.text.contains("<b>bold</b>"));
            assert!(data.text.contains("<i>italic</i>"));
            assert!(data.text.contains("<s>strikethrough</s>"));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_inline_formatting_in_heading() {
        let md = "# **Bold** and _italic_ heading";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Heading { data } = &blocks[0] {
            assert!(data.text.contains("<b>Bold</b>"));
            assert!(data.text.contains("<i>italic</i>"));
            assert_eq!(data.level, 1);
        } else {
            panic!("Expected heading block");
        }
    }

    #[test]
    fn test_inline_formatting_in_list() {
        let md = "- Item with **bold**\n- Item with _italic_\n- Item with ~~strikethrough~~";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.items.len(), 3);
            assert!(data.items[0].contains("<b>bold</b>"));
            assert!(data.items[1].contains("<i>italic</i>"));
            assert!(data.items[2].contains("<s>strikethrough</s>"));
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_inline_formatting_in_blockquote() {
        let md = "> This quote has **bold** and _italic_ text.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Quote { data } = &blocks[0] {
            assert!(data.text.contains("<b>bold</b>"));
            assert!(data.text.contains("<i>italic</i>"));
        } else {
            panic!("Expected quote block");
        }
    }

    #[test]
    fn test_inline_formatting_with_links() {
        let md = "This has **[bold link](https://example.com)** and _[italic link](https://example.com)_.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            // The formatting converts before links, so we get a link wrapped in bold tags
            assert!(data.text.contains("https://example.com"));
            assert!(data.text.contains(r#"target="_blank""#));
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_nested_inline_formatting() {
        let md = "This is **bold with _nested italic_** inside.";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::Paragraph { data } = &blocks[0] {
            // After formatting conversion, we expect both tags applied
            assert!(data.text.contains("<b>"));
            assert!(data.text.contains("<i>"));
        } else {
            panic!("Expected paragraph block");
        }
    }
}
