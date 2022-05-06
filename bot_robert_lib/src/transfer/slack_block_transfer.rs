
use serde::{Serialize, Deserialize};
use crate::transfer::slack_element_transfer::SlackElementTransfer;
use crate::transfer::slack_text_transfer::SlackTextTransfer;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockTransfer {
    #[serde(rename = "type")]
    pub block_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<SlackElementTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<SlackElementTransfer>>, // slack wants plural tag if there are more than one element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<SlackTextTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<SlackTextTransfer>,
}