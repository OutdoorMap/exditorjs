

/// Generates a unique ID for blocks
pub fn generate_block_id() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u128(timestamp);
    let hash = hasher.finish();
    
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();
    
    let mut id = String::new();
    let mut num = hash;
    for _ in 0..10 {
        id.push(chars[(num % 62) as usize]);
        num /= 62;
    }
    id
}

/// Represents an Editor.js block with ID
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EditorJsBlockWithId {
    pub id: String,
    pub data: BlockData,
    #[serde(rename = "type")]
    pub block_type: String,
}

/// Block data wrapper
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(untagged)]
pub enum BlockData {
    Paragraph(ParagraphData),
    Heading(HeadingData),
    List(ListData),
    Image(ImageData),
    Code(CodeData),
    Quote(QuoteData),
    Raw(RawData),
    Table(TableData),
    Delimiter(DelimiterData),
    Embed(EmbedData),
}

/// Represents an Editor.js block (internal representation)
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(tag = "type")]
pub enum EditorJsBlock {
    #[serde(rename = "paragraph")]
    Paragraph { data: ParagraphData },

    #[serde(rename = "heading")]
    Heading { data: HeadingData },

    #[serde(rename = "list")]
    List { data: ListData },

    #[serde(rename = "image")]
    Image { data: ImageData },

    #[serde(rename = "code")]
    Code { data: CodeData },

    #[serde(rename = "quote")]
    Quote { data: QuoteData },

    #[serde(rename = "raw")]
    Raw { data: RawData },

    #[serde(rename = "table")]
    Table { data: TableData },

    #[serde(rename = "delimiter")]
    Delimiter {},

    #[serde(rename = "embed")]
    Embed { data: EmbedData },
}

impl EditorJsBlock {
    /// Convert to block with ID
    pub fn with_id(self) -> EditorJsBlockWithId {
        let (block_type, data) = match self {
            EditorJsBlock::Paragraph { data } => ("paragraph".to_string(), BlockData::Paragraph(data)),
            EditorJsBlock::Heading { data } => ("heading".to_string(), BlockData::Heading(data)),
            EditorJsBlock::List { data } => ("list".to_string(), BlockData::List(data)),
            EditorJsBlock::Image { data } => ("image".to_string(), BlockData::Image(data)),
            EditorJsBlock::Code { data } => ("code".to_string(), BlockData::Code(data)),
            EditorJsBlock::Quote { data } => ("quote".to_string(), BlockData::Quote(data)),
            EditorJsBlock::Raw { data } => ("raw".to_string(), BlockData::Raw(data)),
            EditorJsBlock::Table { data } => ("table".to_string(), BlockData::Table(data)),
            EditorJsBlock::Delimiter {} => ("delimiter".to_string(), BlockData::Delimiter(DelimiterData {})),
            EditorJsBlock::Embed { data } => ("embed".to_string(), BlockData::Embed(data)),
        };

        EditorJsBlockWithId {
            id: generate_block_id(),
            data,
            block_type,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ParagraphData {
    pub text: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct HeadingData {
    pub text: String,
    pub level: u8,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ListMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "counterType", rename = "counterType")]
    pub counter_type: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct ListItemMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ListItem {
    pub content: String,
    pub meta: ListItemMeta,
    pub items: Vec<ListItem>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ListData {
    pub style: String,
    pub items: Vec<ListItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ListMeta>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ImageData {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "withBorder", rename = "withBorder")]
    pub with_border: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "withBackground", rename = "withBackground")]
    pub with_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretched: Option<bool>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CodeData {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct QuoteData {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    pub alignment: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct RawData {
    pub html: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DelimiterData {}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TableData {
    pub content: Vec<Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EmbedData {
    pub service: String,
    pub source: String,
    pub embed: String,
    pub width: u32,
    pub height: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}
