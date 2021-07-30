use serde::{Deserialize, Serialize};
use crate::transfer::slack_text_transfer::SlackTextTransfer;

#[derive(Serialize, Deserialize)]
pub struct SlackElementTransfer {
    #[serde(rename = "type")]
    pub element_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackTextTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<SlackTextTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
}