//! ExditorJS Native
//!
//! A Rust library for converting HTML and Markdown to Editor.js JSON format.
//!
//! # Examples
//!
//! ```no_run
//! use exditorjs_native::{html_to_editorjs, markdown_to_editorjs};
//!
//! let html = "<h1>Hello</h1><p>World</p>";
//! let blocks = html_to_editorjs(html).unwrap();
//! println!("{}", serde_json::to_string(&blocks).unwrap());
//!
//! let markdown = "# Hello\n\nWorld";
//! let blocks = markdown_to_editorjs(markdown).unwrap();
//! println!("{}", serde_json::to_string(&blocks).unwrap());
//! ```

extern crate rustler;

pub mod blocks;
pub mod embed;
pub mod error;
pub mod html;
pub mod markdown;
pub mod models;

pub use embed::{detect_embed_service, detect_service_from_src, parse_iframe};
pub use error::{Error, Result};
pub use html::html_to_editorjs;
pub use markdown::markdown_to_editorjs;
pub use models::{EditorJsBlock, EditorJsBlockWithId};
use rustler::{Encoder, NifResult};

/// Represents an Editor.js document with proper structure
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EditorJsDocument {
    pub time: i64,
    pub blocks: Vec<EditorJsBlockWithId>,
    pub version: String,
}

impl EditorJsDocument {
    /// Create a new Editor.js document from blocks
    pub fn new(blocks: Vec<EditorJsBlock>) -> Self {
        EditorJsDocument {
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            blocks: blocks.into_iter().map(|b| b.with_id()).collect(),
            version: "2.25.0".to_string(),
        }
    }
}

// Rustler atoms
mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}

// NIF function to convert HTML to EditorJS
#[rustler::nif(schedule = "DirtyCpu")]
fn html_to_editorjs_nif(env: rustler::Env<'_>, html: String) -> NifResult<rustler::Term<'_>> {
    match html_to_editorjs(&html) {
        Ok(blocks) => {
            let document = EditorJsDocument::new(blocks);
            match serde_json::to_string(&document) {
                Ok(json) => Ok((atoms::ok(), json).encode(env)),
                Err(_) => Err(rustler::error::Error::RaiseTerm(Box::new(
                    "json_encode_error",
                ))),
            }
        }
        Err(_) => Err(rustler::error::Error::RaiseTerm(Box::new(
            "conversion_error",
        ))),
    }
}

// NIF function to convert Markdown to EditorJS
#[rustler::nif(schedule = "DirtyCpu")]
fn markdown_to_editorjs_nif(
    env: rustler::Env<'_>,
    markdown: String,
) -> NifResult<rustler::Term<'_>> {
    match markdown_to_editorjs(&markdown) {
        Ok(blocks) => {
            let document = EditorJsDocument::new(blocks);
            match serde_json::to_string(&document) {
                Ok(json) => Ok((atoms::ok(), json).encode(env)),
                Err(_) => Err(rustler::error::Error::RaiseTerm(Box::new(
                    "json_encode_error",
                ))),
            }
        }
        Err(_) => Err(rustler::error::Error::RaiseTerm(Box::new(
            "conversion_error",
        ))),
    }
}

rustler::init!("Elixir.ExditorJS");
