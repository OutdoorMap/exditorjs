use exditorjs_native::{markdown_to_editorjs, EditorJsDocument};

fn main() {
    println!("Inline Formatting Example\n");
    println!("=========================\n");

    let markdown = r#"# Text Formatting Guide

This guide demonstrates the various inline formatting options available in markdown.

## Bold Text

Use **double asterisks** to create bold text. This is **important information** you should remember.

## Italic Text

Use _single underscores_ to create italic text. This is _emphasized content_ that stands out.

## Strikethrough Text

Use ~~double tildes~~ to create strikethrough text. This shows ~~incorrect~~ and corrected information.

## Combined Formatting

You can combine multiple formats:
- **Bold and _italic_** text together
- **Bold with ~~strikethrough~~** in same line
- _Italic with ~~strikethrough~~_ combination

## Formatted Lists

- **Feature 1**: This is a critical feature
- _Feature 2_: This is an emphasized feature
- ~~Removed feature~~: This has been deprecated

## Quoted Content with Formatting

> Remember to use **proper formatting** in your documents. _Emphasis_ helps readers understand key points, and ~~outdated~~ information should be clearly marked.

## Links with Formatting

- Visit our **[main website](https://example.com)** for more info
- Check the _[documentation](https://example.com/docs)_ for detailed guides
- ~~[Old link](https://old.example.com)~~ - Please use new links instead

"#;

    println!("Input Markdown:");
    println!("{}\n", markdown);

    match markdown_to_editorjs(markdown) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);

            println!("Output Editor.js JSON:");
            println!("{}\n", serde_json::to_string_pretty(&doc).unwrap());

            println!("\nFormatting Summary:");
            println!("===================");

            let mut bold_count = 0;
            let mut italic_count = 0;
            let mut strikethrough_count = 0;
            let mut link_count = 0;

            for block in doc.blocks.iter() {
                if let serde_json::Value::Object(data) = &block.data {
                    if let Some(text) = data.get("text") {
                        if let serde_json::Value::String(text_str) = text {
                            bold_count += text_str.matches("<b>").count();
                            italic_count += text_str.matches("<i>").count();
                            strikethrough_count += text_str.matches("<s>").count();
                            link_count += text_str.matches("href=").count();
                        }
                    }
                    if let Some(items) = data.get("items") {
                        if let serde_json::Value::Array(item_array) = items {
                            for item in item_array {
                                if let serde_json::Value::String(item_str) = item {
                                    bold_count += item_str.matches("<b>").count();
                                    italic_count += item_str.matches("<i>").count();
                                    strikethrough_count += item_str.matches("<s>").count();
                                    link_count += item_str.matches("href=").count();
                                }
                            }
                        }
                    }
                }
            }

            println!("Bold text instances: {}", bold_count);
            println!("Italic text instances: {}", italic_count);
            println!("Strikethrough instances: {}", strikethrough_count);
            println!("Links: {}", link_count);
        }
        Err(e) => {
            eprintln!("Error converting markdown: {}", e);
        }
    }
}
