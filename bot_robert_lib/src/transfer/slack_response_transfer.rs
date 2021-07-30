use serde::{Serialize, Deserialize};
use crate::transfer::SlackBlockTransfer;

#[derive(Serialize, Deserialize)]
pub struct SlackResponseTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<SlackBlockTransfer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
}