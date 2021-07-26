use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SlackTextTransfer {
    #[serde(rename = "type")]
    pub text_type: String,
    pub text: String,
    pub emoji: bool,
}