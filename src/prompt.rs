use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Prompt {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}
