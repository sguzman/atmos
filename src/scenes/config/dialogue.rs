use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DialogueConfig {
    pub start: String,
    #[serde(default)]
    pub nodes: Vec<DialogueNode>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DialogueNode {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub options: Vec<DialogueOption>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DialogueOption {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(default)]
    pub once: bool,
}
