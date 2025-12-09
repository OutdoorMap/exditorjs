use exditorjs_native::{html_to_editorjs, EditorJsDocument};

fn main() {
    let html = r#"<!doctype html>
<html>
  <head>
    <title>HTML5 Test Page</title>
  </head>
  <body>
    <h1>HTML5 Test Page</h1>
    <p>This is a test page filled with common HTML elements to be used to provide visual feedback whilst building CSS systems and frameworks.</p>
    
    <h2>Headings</h2>
    <h1>Heading 1</h1>
    <h2>Heading 2</h2>
    <h3>Heading 3</h3>
    <h4>Heading 4</h4>
    <h5>Heading 5</h5>
    <h6>Heading 6</h6>
    
    <h2>Paragraphs</h2>
    <p>A paragraph (from the Greek paragraphos, "to write beside" or "written beside") is a self-contained unit of a discourse in writing dealing with a particular point or idea.</p>
    
    <h2>Blockquotes</h2>
    <blockquote>
      <p>A block quotation (also known as a long quotation or extract) is a quotation in a written document, that is set off from the main text as a paragraph, or block of text.</p>
    </blockquote>
    
    <h2>Lists</h2>
    <h3>Unordered List</h3>
    <ul>
      <li>List Item 1</li>
      <li>List Item 2</li>
      <li>List Item 3</li>
    </ul>
    
    <h3>Ordered List</h3>
    <ol>
      <li>List Item 1</li>
      <li>List Item 2</li>
      <li>List Item 3</li>
    </ol>
    
    <h2>Horizontal Rule</h2>
    <hr>
    
    <h2>Tables</h2>
    <table>
      <thead>
        <tr>
          <th>Heading 1</th>
          <th>Heading 2</th>
          <th>Heading 3</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Cell 1</td>
          <td>Cell 2</td>
          <td>Cell 3</td>
        </tr>
        <tr>
          <td>Cell 4</td>
          <td>Cell 5</td>
          <td>Cell 6</td>
        </tr>
      </tbody>
    </table>
    
    <h2>Code</h2>
    <pre><code>P R E F O R M A T T E D T E X T
  ! " # $ % & ' ( ) * + , - . /
  0 1 2 3 4 5 6 7 8 9 : ; < = > ?</code></pre>
    
    <h2>Inline Elements</h2>
    <p><a href="http://example.com">This is a text link</a>.</p>
    <p><strong>Strong is used to indicate strong importance.</strong></p>
    <p><em>This text has added emphasis.</em></p>
    <p>The <b>b element</b> is stylistically different text from normal text, without any special importance.</p>
    <p>The <i>i element</i> is text that is offset from the normal text.</p>
    <p><del>This text is deleted</del> and <ins>This text is inserted</ins>.</p>
    <p><s>This text has a strikethrough</s>.</p>
    
    <h2>Images</h2>
    <p><img src="http://placekitten.com/480/480" alt="Image alt text"></p>
    <figure>
      <img src="http://placekitten.com/420/420" alt="Image alt text">
      <figcaption>Here is a caption for this image.</figcaption>
    </figure>
    
    <footer>
      <p>Made by <a href="http://twitter.com/cbracco">@cbracco</a>. Code on <a href="http://github.com/cbracco/html5-test-page">GitHub</a>.</p>
    </footer>
  </body>
</html>"#;

    println!("Converting HTML to Editor.js...\n");

    match html_to_editorjs(html) {
        Ok(blocks) => {
            let doc = EditorJsDocument::new(blocks);

            println!("✅ Conversion successful!\n");
            println!("Document Statistics:");
            println!("  - Total blocks: {}", doc.blocks.len());
            println!("  - Version: {}", doc.version);
            println!("  - Timestamp: {}", doc.time);
            println!("\nBlocks by type:");

            let mut block_types: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for block in &doc.blocks {
                *block_types.entry(block.block_type.clone()).or_insert(0) += 1;
            }

            for (block_type, count) in &block_types {
                println!("  - {}: {}", block_type, count);
            }

            println!("\n========================================");
            println!("Full JSON Output:");
            println!("========================================\n");

            match serde_json::to_string_pretty(&doc) {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("Error serializing to JSON: {}", e),
            }
        }
        Err(e) => {
            eprintln!("❌ Conversion failed: {}", e);
        }
    }
}
