use exditorjs_native::{markdown_to_editorjs, EditorJsDocument};

fn main() {
    println!("Markdown Link Conversion Example\n");
    println!("==================================\n");

    let markdown = r#"# Learn Rust Programming

Welcome to our comprehensive guide on Rust programming. This documentation will help you get started quickly.

## Quick Links

- Read [The Rust Book](https://doc.rust-lang.org/book/) - Official language guide
- Explore [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through examples
- Browse [Crates.io](https://crates.io) - The Rust package registry

## Getting Help

> Check out [Stack Overflow](https://stackoverflow.com/questions/tagged/rust) for community support. You can also visit the [Rust Community](https://www.rust-lang.org/community) page for more resources.

For more information, visit [The Official Rust Website](https://www.rust-lang.org)."#;

    println!("Input Markdown:");
    println!("{}\n", markdown);

    match markdown_to_editorjs(markdown) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);

            println!("Output Editor.js JSON:");
            println!("{}\n", serde_json::to_string_pretty(&doc).unwrap());

            println!("Block Summary:");
            println!("==============");
            for (i, block) in doc.blocks.iter().enumerate() {
                println!(
                    "\nBlock {}: ID={}, Type={}",
                    i + 1,
                    block.id,
                    block.block_type
                );

                match block.block_type.as_str() {
                    "heading" => {
                        if let serde_json::Value::Object(data) = &block.data {
                            if let Some(text) = data.get("text") {
                                println!("  Text: {}", text);
                            }
                            if let Some(level) = data.get("level") {
                                println!("  Level: {}", level);
                            }
                        }
                    }
                    "paragraph" => {
                        if let serde_json::Value::Object(data) = &block.data {
                            if let Some(text) = data.get("text") {
                                let text_str = text.as_str().unwrap_or("");
                                if text_str.contains("<a") {
                                    println!("  Contains links: YES");
                                    // Extract link info
                                    let re =
                                        regex::Regex::new(r#"href="([^"]+)"[^>]*>([^<]+)</a>"#)
                                            .unwrap();
                                    for cap in re.captures_iter(text_str) {
                                        println!("    - Link: {} -> {}", &cap[2], &cap[1]);
                                    }
                                } else {
                                    println!("  Text: {}", text);
                                }
                            }
                        }
                    }
                    "list" => {
                        if let serde_json::Value::Object(data) = &block.data {
                            if let Some(items) = data.get("items").and_then(|v| v.as_array()) {
                                println!("  Items: {}", items.len());
                                for (j, item) in items.iter().enumerate() {
                                    if let Some(item_str) = item.as_str() {
                                        if item_str.contains("<a") {
                                            println!("    [{}] Contains link", j + 1);
                                        } else {
                                            println!("    [{}] {}", j + 1, item_str);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "quote" => {
                        if let serde_json::Value::Object(data) = &block.data {
                            if let Some(text) = data.get("text") {
                                let text_str = text.as_str().unwrap_or("");
                                if text_str.contains("<a") {
                                    println!("  Contains links: YES");
                                } else {
                                    println!("  Text: {}", text);
                                }
                            }
                        }
                    }
                    _ => println!("  Type: {}", block.block_type),
                }
            }
        }
        Err(e) => eprintln!("Error converting markdown: {}", e),
    }
}
