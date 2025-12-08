use exditorjs_native::markdown_to_editorjs;
use serde_json::json;

fn main() {
    let swedish_text = r#"En av de äldsta biograferna i Sverige men med den senaste tekniken inom ljud och bild. I den glamoröst inredda biosalongen kan du njuta av 3-D filmer, teater, live-sänd opera och andra event. 
Stora salongen har den senaste tekniken inom ljud och bild. Med den digitala installationen öppnar sig en ny värld med att kunna visa 3-D filmer och livesända olika event som t ex opera ifrån Metropolitan, teater ifrån National Theatre eller dramaten. Sportevenemang och konserter. 
För aktuellt program se hemsidan."#;

    println!("=== Input Text ===");
    println!("{}\n", swedish_text);

    match markdown_to_editorjs(swedish_text) {
        Ok(blocks) => {
            println!("=== Conversion Result ===");
            println!("Number of blocks: {}\n", blocks.len());
            
            for (i, block) in blocks.iter().enumerate() {
                println!("Block {}: {}", i + 1, serde_json::to_string_pretty(&block).unwrap());
                println!();
            }

            println!("=== Full JSON Output ===");
            println!("{}", serde_json::to_string_pretty(&blocks).unwrap());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
