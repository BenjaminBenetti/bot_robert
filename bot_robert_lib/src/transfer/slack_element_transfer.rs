use serde::{Deserialize, Serialize};
use crate::transfer::slack_text_transfer::SlackTextTransfer;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackElementTransfer {
    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    pub element_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackTextTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<SlackTextTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SlackElementTransfer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_options: Option<Vec<SlackElementTransfer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
}