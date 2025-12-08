use crate::error::{Error, Result};
use crate::models::*;
use regex::Regex;
use std::collections::VecDeque;

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
        let _html = self.html.trim();

        // Simple regex-based HTML parsing
        let re = Regex::new(r"<([^/>]+)([^>]*)>(.*?)</\1>").unwrap();
        let mut last_end = 0;

        for cap in re.captures_iter(&self.html) {
            let tag = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let attrs = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let content = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let full_match = cap.get(0).unwrap();

            // Handle text before the tag
            if last_end < full_match.start() {
                let text = &self.html[last_end..full_match.start()].trim();
                if !text.is_empty() {
                    blocks.push(EditorJsBlock::Paragraph {
                        data: ParagraphData {
                            text: self.clean_html(text),
                        },
                    });
                }
            }

            // Parse the tag
            let block = self.parse_tag(tag, attrs, content)?;
            if let Some(b) = block {
                blocks.push(b);
            }

            last_end = full_match.end();
        }

        // Handle remaining text
        if last_end < self.html.len() {
            let text = &self.html[last_end..].trim();
            if !text.is_empty() {
                blocks.push(EditorJsBlock::Paragraph {
                    data: ParagraphData {
                        text: self.clean_html(text),
                    },
                });
            }
        }

        // If no blocks were created, treat entire HTML as raw
        if blocks.is_empty() {
            blocks.push(EditorJsBlock::Raw {
                data: RawData {
                    html: self.html.clone(),
                },
            });
        }

        Ok(blocks)
    }

    fn parse_tag(&self, tag: &str, attrs: &str, content: &str) -> Result<Option<EditorJsBlock>> {
        let content = content.trim();

        match tag.to_lowercase().as_str() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = tag.chars().nth(1).unwrap().to_digit(10).unwrap_or(1) as u8;
                Ok(Some(EditorJsBlock::Heading {
                    data: HeadingData {
                        text: self.clean_html(content),
                        level,
                    },
                }))
            }
            "p" => Ok(Some(EditorJsBlock::Paragraph {
                data: ParagraphData {
                    text: self.clean_html(content),
                },
            })),
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
            "ul" => Ok(Some(self.parse_list(content, "unordered")?)),
            "ol" => Ok(Some(self.parse_list(content, "ordered")?)),
            "img" => Ok(Some(self.parse_image(attrs)?)),
            "table" => Ok(Some(self.parse_table(content)?)),
            "hr" => Ok(Some(EditorJsBlock::Delimiter {})),
            _ => Ok(None),
        }
    }

    fn parse_list(&self, content: &str, style: &str) -> Result<EditorJsBlock> {
        let re = Regex::new(r"<li[^>]*>(.*?)</li>").unwrap();
        let items: Vec<String> = re
            .captures_iter(content)
            .filter_map(|cap| cap.get(1).map(|m| self.clean_html(m.as_str())))
            .collect();

        Ok(EditorJsBlock::List {
            data: ListData {
                style: style.to_string(),
                items,
            },
        })
    }

    fn parse_image(&self, attrs: &str) -> Result<EditorJsBlock> {
        let src_re = Regex::new(r#"src=["']?([^"'\s>]+)["']?"#).unwrap();
        let alt_re = Regex::new(r#"alt=["']?([^"'\s>]+)["']?"#).unwrap();

        let url = src_re
            .captures(attrs)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        let caption = alt_re
            .captures(attrs)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string());

        Ok(EditorJsBlock::Image {
            data: ImageData {
                url,
                caption,
                withBorder: None,
                withBackground: None,
                stretched: None,
            },
        })
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

    fn clean_html(&self, text: &str) -> String {
        // Remove HTML tags
        let re = Regex::new(r"<[^>]+>").unwrap();
        let cleaned = re.replace_all(text, "");

        // Decode HTML entities
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
    fn test_parse_list() {
        let html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        let blocks = html_to_editorjs(html).unwrap();
        assert_eq!(blocks.len(), 1);
    }
}
