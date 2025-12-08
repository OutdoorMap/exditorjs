/// Element parser module for handling individual block elements
use crate::models::*;

pub trait ElementParser {
    fn parse(&self) -> Option<EditorJsBlock>;
}
