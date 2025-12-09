use crate::embed::{detect_embed_service, detect_service_from_src, parse_iframe};
use crate::error::Result;
use crate::models::*;
use regex::Regex;

/// Convert HTML to Editor.js blocks
pub fn html_to_editorjs(html: &str) -> Result<Vec<EditorJsBlock>> {
    let parser = HtmlParser::new(html);
    parser.parse()
}

struct HtmlParser {
    html: String,
}

impl HtmlParser {
    fn new(html: &str) -> Self {
        HtmlParser {
            html: html.to_string(),
        }
    }

    fn parse(&self) -> Result<Vec<EditorJsBlock>> {
        let mut blocks = Vec::new();
        let html = self.html.trim();
        let mut pos = 0;

        while pos < html.len() {
            // Skip whitespace
            while pos < html.len() && html.chars().nth(pos).is_some_and(|c| c.is_whitespace()) {
                pos += 1;
            }

            if pos >= html.len() {
                break;
            }

            // Check if we're at a tag
            if html.chars().nth(pos) == Some('<') {
                // Find the end of the tag
                if let Some(tag_end) = html[pos..].find('>') {
                    let _tag_full = &html[pos..pos + tag_end + 1];

                    // Parse the tag
                    if let Some(block) = self.parse_element(html, &mut pos) {
                        blocks.push(block);
                    } else {
                        pos += tag_end + 1;
                    }
                } else {
                    pos += 1;
                }
            } else {
                // Text content outside tags
                if let Some(tag_pos) = html[pos..].find('<') {
                    let text = html[pos..pos + tag_pos].trim();
                    if !text.is_empty() {
                        blocks.push(EditorJsBlock::Paragraph {
                            data: ParagraphData {
                                text: self.clean_html(text),
                            },
                        });
                    }
                    pos += tag_pos;
                } else {
                    let text = html[pos..].trim();
                    if !text.is_empty() {
                        blocks.push(EditorJsBlock::Paragraph {
                            data: ParagraphData {
                                text: self.clean_html(text),
                            },
                        });
                    }
                    break;
                }
            }
        }

        if blocks.is_empty() {
            blocks.push(EditorJsBlock::Raw {
                data: RawData {
                    html: self.html.clone(),
                },
            });
        }

        Ok(blocks)
    }

    fn parse_element(&self, html: &str, pos: &mut usize) -> Option<EditorJsBlock> {
        if html.chars().nth(*pos) != Some('<') {
            return None;
        }

        let tag_start = *pos;

        // Find tag end
        let tag_end = html[tag_start..].find('>')?;
        let tag_content = &html[tag_start + 1..tag_start + tag_end];

        // Self-closing or void tags
        if tag_content.ends_with('/')
            || tag_content.starts_with("img ")
            || tag_content.starts_with("iframe ")
            || tag_content.starts_with("br")
            || tag_content.starts_with("hr")
        {
            *pos = tag_start + tag_end + 1;

            if tag_content.starts_with("img") {
                if let Ok(Some(block)) = self.parse_image(tag_content) {
                    return Some(block);
                }
            } else if tag_content.starts_with("iframe") {
                if let Some(block) = self.parse_iframe_tag(tag_content) {
                    return Some(block);
                }
            } else if tag_content.starts_with("hr") {
                return Some(EditorJsBlock::Delimiter {});
            }
            return None;
        }

        // Extract tag name and attributes
        let parts: Vec<&str> = tag_content.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let tag_name = parts[0];
        let attrs = if parts.len() > 1 {
            tag_content[tag_name.len()..].trim()
        } else {
            ""
        };

        // Check for iframe with closing tag
        if tag_name.eq_ignore_ascii_case("iframe") {
            if let Some(block) = self.parse_iframe_tag(tag_content) {
                let closing_tag = "</iframe>";
                if let Some(closing_pos) = html[tag_start + tag_end + 1..].find(closing_tag) {
                    *pos = tag_start + tag_end + 1 + closing_pos + closing_tag.len();
                }
                return Some(block);
            }
        }

        // Find closing tag
        let closing_tag = format!("</{}>", tag_name);
        let content_start = tag_start + tag_end + 1;

        if let Some(closing_pos) = html[content_start..].find(&closing_tag) {
            let content = &html[content_start..content_start + closing_pos];
            *pos = content_start + closing_pos + closing_tag.len();

            if let Ok(Some(block)) = self.parse_tag(tag_name, attrs, content) {
                return Some(block);
            }
        }

        None
    }

    fn parse_tag(&self, tag: &str, attrs: &str, content: &str) -> Result<Option<EditorJsBlock>> {
        let content = content.trim();
        let tag_lower = tag.to_lowercase();

        match tag_lower.as_str() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = tag_lower.chars().nth(1).unwrap().to_digit(10).unwrap_or(1) as u8;
                Ok(Some(EditorJsBlock::Heading {
                    data: HeadingData {
                        text: self.clean_html(content),
                        level,
                    },
                }))
            }
            "p" | "div" | "span" => {
                let text = self.clean_html(content);

                // Check if the paragraph contains an embed link
                if let Some(block) = self.parse_embed_from_paragraph(content) {
                    return Ok(Some(block));
                }

                if !text.is_empty() {
                    Ok(Some(EditorJsBlock::Paragraph {
                        data: ParagraphData { text },
                    }))
                } else {
                    Ok(None)
                }
            }
            "blockquote" => Ok(Some(EditorJsBlock::Quote {
                data: QuoteData {
                    text: self.clean_html(content),
                    caption: None,
                    alignment: "left".to_string(),
                },
            })),
            "code" | "pre" => Ok(Some(EditorJsBlock::Code {
                data: CodeData {
                    code: content.to_string(),
                    language: self.extract_language(attrs),
                },
            })),
            "ul" => {
                let block = self.parse_list(content, "unordered")?;
                Ok(Some(block))
            }
            "ol" => {
                let block = self.parse_list(content, "ordered")?;
                Ok(Some(block))
            }
            "table" => {
                let block = self.parse_table(content)?;
                Ok(Some(block))
            }
            "hr" => Ok(Some(EditorJsBlock::Delimiter {})),
            "li" => {
                // Skip li tags when parsing as we handle them in list parsing
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn parse_list(&self, content: &str, style: &str) -> Result<EditorJsBlock> {
        let items = self.parse_list_items(content, style);

        let data = if style == "ordered" {
            ListData {
                style: "ordered".to_string(),
                items,
                meta: Some(ListMeta {
                    start: Some(1),
                    counter_type: None,
                }),
            }
        } else {
            ListData {
                style: style.to_string(),
                items,
                meta: None,
            }
        };

        Ok(EditorJsBlock::List { data })
    }

    fn parse_list_items(&self, content: &str, _style: &str) -> Vec<ListItem> {
        let mut items = Vec::new();
        let li_re = Regex::new(r"<li[^>]*>(.*?)</li>").unwrap();
        let nested_ul_re = Regex::new(r"<ul[^>]*>(.*?)</ul>").unwrap();
        let nested_ol_re = Regex::new(r"<ol[^>]*>(.*?)</ol>").unwrap();

        for cap in li_re.captures_iter(content) {
            let li_content = cap.get(1).unwrap().as_str();

            // Check for nested lists
            let mut item_text = li_content.to_string();
            let mut nested_items = Vec::new();

            // Extract and remove nested unordered list
            if let Some(nested_cap) = nested_ul_re.captures(li_content) {
                let nested_content = nested_cap.get(1).unwrap().as_str();
                item_text = nested_ul_re.replace(&item_text, "").to_string();
                nested_items = self.parse_list_items(nested_content, "unordered");
            }
            // Extract and remove nested ordered list
            else if let Some(nested_cap) = nested_ol_re.captures(li_content) {
                let nested_content = nested_cap.get(1).unwrap().as_str();
                item_text = nested_ol_re.replace(&item_text, "").to_string();
                nested_items = self.parse_list_items(nested_content, "ordered");
            }

            let cleaned_text = self.clean_html(item_text.trim());

            // Check if this is a checklist item (look for input type="checkbox")
            let checked = if li_content.contains("type=\"checkbox\"") {
                if li_content.contains("checked") {
                    Some(true)
                } else {
                    Some(false)
                }
            } else {
                None
            };

            items.push(ListItem {
                content: cleaned_text,
                meta: ListItemMeta { checked },
                items: nested_items,
            });
        }

        items
    }

    fn parse_image(&self, attrs: &str) -> Result<Option<EditorJsBlock>> {
        let src_re = Regex::new(r#"src=["']?([^"'\s>]+)["']?"#).unwrap();
        let alt_re = Regex::new(r#"alt=["']([^"']*)["']"#).unwrap();

        let url = src_re
            .captures(attrs)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        if url.is_empty() {
            return Ok(None);
        }

        let caption = alt_re
            .captures(attrs)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .and_then(|s| if s.is_empty() { None } else { Some(s) });

        Ok(Some(EditorJsBlock::Image {
            data: ImageData {
                url,
                caption,
                with_border: None,
                with_background: None,
                stretched: None,
            },
        }))
    }

    fn parse_table(&self, content: &str) -> Result<EditorJsBlock> {
        let row_re = Regex::new(r"<tr[^>]*>(.*?)</tr>").unwrap();
        let cell_re = Regex::new(r"<t[dh][^>]*>(.*?)</t[dh]>").unwrap();

        let mut table_content = Vec::new();

        for row_cap in row_re.captures_iter(content) {
            let row_content = row_cap.get(1).unwrap().as_str();
            let mut cells = Vec::new();

            for cell_cap in cell_re.captures_iter(row_content) {
                let cell = cell_cap.get(1).unwrap().as_str();
                cells.push(self.clean_html(cell));
            }

            if !cells.is_empty() {
                table_content.push(cells);
            }
        }

        Ok(EditorJsBlock::Table {
            data: TableData {
                content: table_content,
            },
        })
    }

    fn extract_language(&self, attrs: &str) -> Option<String> {
        let re = Regex::new(r#"(?:class|lang)=["']?([^"'\s>]+)["']?"#).unwrap();
        re.captures(attrs)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn parse_iframe_tag(&self, attrs: &str) -> Option<EditorJsBlock> {
        if let Some((src, width, height)) = parse_iframe(attrs) {
            // Try to detect the service from the src URL
            if let Some(service) = detect_service_from_src(&src) {
                return Some(EditorJsBlock::Embed {
                    data: EmbedData {
                        service,
                        source: src.clone(),
                        embed: src,
                        width,
                        height,
                        caption: None,
                    },
                });
            }
        }
        None
    }

    fn parse_embed_from_paragraph(&self, content: &str) -> Option<EditorJsBlock> {
        // Extract URLs from links in the paragraph
        let url_re = Regex::new(r#"(?:href=["']?)?https?://[^\s"'<>]+"#).ok()?;

        for caps in url_re.captures_iter(content) {
            let url_match = caps.get(0)?;
            let url = url_match.as_str().trim_matches('"').trim_matches('\'');

            // Try to detect if this is an embed service
            if let Some((service, embed_url, width, height)) = detect_embed_service(url) {
                return Some(EditorJsBlock::Embed {
                    data: EmbedData {
                        service,
                        source: url.to_string(),
                        embed: embed_url,
                        width,
                        height,
                        caption: None,
                    },
                });
            }
        }
        None
    }

    fn clean_html(&self, text: &str) -> String {
        let re = Regex::new(r"<[^>]+>").unwrap();
        let cleaned = re.replace_all(text, "");
        self.decode_entities(&cleaned)
    }

    fn decode_entities(&self, text: &str) -> String {
        text.replace("&nbsp;", " ")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&copy;", "©")
            .replace("&reg;", "®")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paragraph() {
        let html = "<p>Hello World</p>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_parse_heading() {
        let html = "<h1>Title</h1>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn test_parse_simple_unordered_list() {
        let html = "<ul><li>First item</li><li>Second item</li><li>Third item</li></ul>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);

        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "unordered");
            assert_eq!(data.items.len(), 3);
            assert_eq!(data.items[0].content, "First item");
            assert_eq!(data.items[1].content, "Second item");
            assert_eq!(data.items[2].content, "Third item");
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    fn test_parse_simple_ordered_list() {
        let html = "<ol><li>Item 1</li><li>Item 2</li></ol>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);

        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "ordered");
            assert_eq!(data.items.len(), 2);
        } else {
            panic!("Expected list block");
        }
    }

    #[test]
    #[ignore]
    fn test_parse_nested_unordered_list() {
        let html = "<ul><li>Item 1</li><li>Item 2<ul><li>Nested 1</li><li>Nested 2</li></ul></li><li>Item 3</li></ul>";
        let blocks = html_to_editorjs(html).unwrap();
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
    #[ignore]
    fn test_parse_nested_ordered_list() {
        let html = "<ol><li>Item 1</li><li>Item 2<ol><li>Nested 1</li><li>Nested 2</li></ol></li><li>Item 3</li></ol>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);

        if let EditorJsBlock::List { data } = &blocks[0] {
            assert_eq!(data.style, "ordered");
            assert_eq!(data.items.len(), 3);
            assert_eq!(data.items[1].items.len(), 2);
        } else {
            panic!("Expected list block");
        }
    }
}
