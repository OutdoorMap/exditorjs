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

            // Check for lists (unordered, ordered, or checklist)
            if self.is_list_start(line) {
                let (list_block, next_i) = self.parse_list(i);
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

    fn is_list_start(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        
        // Unordered list: -, +, *
        if trimmed.starts_with("- ") || trimmed.starts_with("+ ") || trimmed.starts_with("* ") {
            return true;
        }
        
        // Ordered list: 1. 2. etc
        if let Some(dot_pos) = trimmed.find(". ") {
            if trimmed[..dot_pos].chars().all(|c| c.is_numeric()) {
                return true;
            }
        }
        
        // Checklist: - [ ] or - [x]
        if trimmed.starts_with("- [") || trimmed.starts_with("+ [") || trimmed.starts_with("* [") {
            return true;
        }
        
        false
    }

    fn get_list_indent(&self, line: &str) -> usize {
        line.len() - line.trim_start().len()
    }

    fn parse_list(&self, start: usize) -> (EditorJsBlock, usize) {
        let first_line = &self.lines[start];
        let trimmed = first_line.trim_start();
        
        let style = if self.is_checklist_item(trimmed) {
            "checklist"
        } else if self.is_ordered_list_item(trimmed) {
            "ordered"
        } else {
            "unordered"
        };

        let (items, next_i) = self.parse_list_items(start, style);
        
        let data = if style == "ordered" {
            ListData {
                style: "ordered".to_string(),
                items,
                meta: Some(ListMeta {
                    start: Some(1),
                    counterType: None,
                }),
            }
        } else {
            ListData {
                style: style.to_string(),
                items,
                meta: None,
            }
        };

        (EditorJsBlock::List { data }, next_i)
    }

    fn is_checklist_item(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.contains("- [") || trimmed.contains("+ [") || trimmed.contains("* [")
    }

    fn is_ordered_list_item(&self, line: &str) -> bool {
        if let Some(dot_pos) = line.find(". ") {
            line[..dot_pos].chars().all(|c| c.is_numeric())
        } else {
            false
        }
    }

    fn parse_list_items(&self, start: usize, style: &str) -> (Vec<ListItem>, usize) {
        let mut items = Vec::new();
        let mut i = start;
        let base_indent = self.get_list_indent(&self.lines[start]);

        while i < self.lines.len() {
            let line = &self.lines[i];
            
            if line.is_empty() {
                i += 1;
                continue;
            }

            let current_indent = self.get_list_indent(line);
            let trimmed = line.trim_start();

            // Check if this line is part of the current list
            if !self.is_list_start(line) {
                break;
            }

            // If indentation is less than base, we've finished this list
            if current_indent < base_indent && !trimmed.is_empty() {
                break;
            }

            // If same indent level as base
            if current_indent == base_indent {
                let (item, next_i) = self.parse_list_item(i, style);
                items.push(item);
                i = next_i;
            } else if current_indent > base_indent {
                // Nested item - add to last item
                if let Some(last_item) = items.last_mut() {
                    let (nested_items, next_i) = self.parse_list_items(i, style);
                    last_item.items = nested_items;
                    i = next_i;
                } else {
                    i += 1;
                }
            } else {
                break;
            }
        }

        (items, i)
    }

    fn parse_list_item(&self, index: usize, style: &str) -> (ListItem, usize) {
        let line = &self.lines[index];
        let trimmed = line.trim_start();

        let (content, checked) = if style == "checklist" {
            self.parse_checklist_item(trimmed)
        } else if style == "ordered" {
            (self.parse_ordered_list_item(trimmed), None)
        } else {
            (self.parse_unordered_list_item(trimmed), None)
        };

        let mut formatted_content = self.convert_inline_formatting(&content);
        formatted_content = self.convert_markdown_links(&formatted_content);

        let meta = if let Some(checked_val) = checked {
            Some(ListItemMeta {
                checked: Some(checked_val),
            })
        } else {
            Some(ListItemMeta { checked: None })
        };

        (
            ListItem {
                content: formatted_content,
                meta: meta.unwrap_or_default(),
                items: Vec::new(),
            },
            index + 1,
        )
    }

    fn parse_unordered_list_item(&self, line: &str) -> String {
        if line.starts_with("- ") {
            line[2..].trim().to_string()
        } else if line.starts_with("+ ") {
            line[2..].trim().to_string()
        } else if line.starts_with("* ") {
            line[2..].trim().to_string()
        } else {
            line.to_string()
        }
    }

    fn parse_ordered_list_item(&self, line: &str) -> String {
        if let Some(dot_pos) = line.find(". ") {
            line[dot_pos + 2..].trim().to_string()
        } else {
            line.to_string()
        }
    }

    fn parse_checklist_item(&self, line: &str) -> (String, Option<bool>) {
        // Pattern: - [ ] text or - [x] text or - [X] text
        let re = Regex::new(r"^[*+-]\s+\[([^\]]*)\]\s+(.*)$").unwrap();
        if let Some(cap) = re.captures(line) {
            let checkbox = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let content = cap.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
            let checked = checkbox.to_lowercase() == "x";
            (content, Some(checked))
        } else {
            (line.to_string(), Some(false))
        }
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
        
        text = self.convert_inline_formatting(&text);
        text = self.convert_markdown_links(&text);
        text
    }

    fn parse_code_block(&self, start: usize) -> (Option<EditorJsBlock>, usize) {
        let mut code_lines = Vec::new();
        let mut language = String::new();
        let mut i = start + 1;

        let first_line = &self.lines[start];
        if first_line.len() > 3 {
            language = first_line[3..].trim().to_string();
        }

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

    fn parse_blockquote(&self, start: usize) -> (EditorJsBlock, usize) {
        let mut lines = Vec::new();
        let mut i = start;

        while i < self.lines.len() && self.lines[i].trim_start().starts_with("> ") {
            let line = self.lines[i].trim_start();
            lines.push(line[2..].trim().to_string());
            i += 1;
        }

        let mut text = lines.join(" ");
        text = self.convert_inline_formatting(&text);
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
        let re = Regex::new(r#"!\[([^\]]*)\]\(([^)\s]+)(?:\s+['\"]([^'\"]*)['\"])?\)"#).unwrap();
        if let Some(cap) = re.captures(line) {
            let alt_text = cap.get(1).map(|m| m.as_str().to_string());
            let url = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
            let title = cap.get(3).map(|m| m.as_str().to_string());

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
        text = self.convert_inline_formatting(&text);
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
    fn test_parse_unordered_list() {
        let md = "- Item 1\n- Item 2\n- Item 3";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "unordered");
            assert_eq!(data.items.len(), 3);
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_parse_ordered_list() {
        let md = "1. Item 1\n2. Item 2\n3. Item 3";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "ordered");
            assert_eq!(data.items.len(), 3);
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_parse_nested_unordered_list() {
        let md = "- Item 1\n- Item 2\n    - Nested 1\n    - Nested 2\n- Item 3";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "unordered");
            assert_eq!(data.items.len(), 3);
            assert_eq!(data.items[1].items.len(), 2);
            assert_eq!(data.items[1].items[0].content, "Nested 1");
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_parse_nested_ordered_list() {
        let md = "1. Item 1\n2. Item 2\n    1. Nested 1\n    2. Nested 2\n3. Item 3";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "ordered");
            assert_eq!(data.items.len(), 3);
            assert_eq!(data.items[1].items.len(), 2);
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_parse_checklist() {
        let md = "- [ ] Unchecked\n- [x] Checked\n- [ ] Unchecked 2";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "checklist");
            assert_eq!(data.items.len(), 3);
            assert_eq!(data.items[0].meta.checked, Some(false));
            assert_eq!(data.items[1].meta.checked, Some(true));
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_list_with_formatting() {
        let md = "- **Bold** item\n- _Italic_ item\n- ~~Strikethrough~~ item";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert!(data.items[0].content.contains("<b>Bold</b>"));
            assert!(data.items[1].content.contains("<i>Italic</i>"));
            assert!(data.items[2].content.contains("<s>Strikethrough</s>"));
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_list_with_links() {
        let md = "- [Link 1](https://example.com)\n- [Link 2](https://example.com)";
        let blocks = markdown_to_editorjs(md).unwrap();
        assert_eq!(blocks.len(), 1);
        
        if let EditorJsBlock::List { data } = &blocks[0] {
            assert!(data.items[0].content.contains(r#"<a href="https://example.com" target="_blank">Link 1</a>"#));
            assert!(data.items[1].content.contains(r#"<a href="https://example.com" target="_blank">Link 2</a>"#));
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_parse_heading() {
        let md = "# Hello";
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
