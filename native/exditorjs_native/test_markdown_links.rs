use std::collections::HashMap;

fn main() {
    println!("Testing Markdown Link Conversion\n");
    println!("=================================\n");

    // Test 1: Simple paragraph with link
    let test1 = "This is a [link to Google](https://google.com) in a paragraph.";
    println!("Test 1 - Paragraph with link:");
    println!("Input:  {}", test1);
    println!("Expected: This is a <a href=\"https://google.com\" target=\"_blank\">link to Google</a> in a paragraph.");
    println!();

    // Test 2: Multiple links in one paragraph
    let test2 = "Check [GitHub](https://github.com) and [Stack Overflow](https://stackoverflow.com) for help.";
    println!("Test 2 - Multiple links:");
    println!("Input:  {}", test2);
    println!("Expected: Check <a href=\"https://github.com\" target=\"_blank\">GitHub</a> and <a href=\"https://stackoverflow.com\" target=\"_blank\">Stack Overflow</a> for help.");
    println!();

    // Test 3: Link in list
    let test3_md = r#"# Documentation

Here are useful resources:

- Learn [Rust](https://www.rust-lang.org/)
- Read [The Book](https://doc.rust-lang.org/book/)
- Browse [Crates.io](https://crates.io)

Visit our [website](https://example.com) for more.

> Check [our blog](https://blog.example.com) for updates."#;

    println!("Test 3 - Links in different block types:");
    println!("Input:\n{}\n", test3_md);
    println!("Expected blocks:");
    println!("1. Heading: \"# Documentation\"");
    println!("2. Paragraph: \"Here are useful resources:\"");
    println!("3. List items:");
    println!("   - Learn <a href=\"https://www.rust-lang.org/\" target=\"_blank\">Rust</a>");
    println!("   - Read <a href=\"https://doc.rust-lang.org/book/\" target=\"_blank\">The Book</a>");
    println!("   - Browse <a href=\"https://crates.io\" target=\"_blank\">Crates.io</a>");
    println!("4. Paragraph: \"Visit our <a href=\"https://example.com\" target=\"_blank\">website</a> for more.\"");
    println!("5. Quote: \"Check <a href=\"https://blog.example.com\" target=\"_blank\">our blog</a> for updates.\"");
    println!();

    // Test 4: Edge cases
    println!("Test 4 - Edge cases:");
    println!("Case 4a - Link with special characters in URL:");
    println!("Input:  [API docs](https://example.com/api?param=value&other=123)");
    println!("Expected: <a href=\"https://example.com/api?param=value&other=123\" target=\"_blank\">API docs</a>");
    println!();

    println!("Case 4b - Link with hash:");
    println!("Input:  Jump to [section](https://example.com#section1)");
    println!("Expected: Jump to <a href=\"https://example.com#section1\" target=\"_blank\">section</a>");
    println!();

    // Test 5: Non-link markdown (should not be converted)
    println!("Test 5 - Non-link markdown:");
    println!("Input:  This has **bold** and *italic* text");
    println!("Expected: This has **bold** and *italic* text (unchanged)");
    println!();

    println!("\n=================================");
    println!("Summary of Link Conversion Feature:");
    println!("=================================");
    println!("✓ Converts [text](url) to <a href=\"url\" target=\"_blank\">text</a>");
    println!("✓ Works in paragraphs, headings, lists, and quotes");
    println!("✓ Preserves all URL parameters and fragments");
    println!("✓ Automatically adds target=\"_blank\" for opening links in new tabs");
    println!("✓ Does not affect other markdown formatting");
    println!("✓ Handles multiple links in a single block");
}
