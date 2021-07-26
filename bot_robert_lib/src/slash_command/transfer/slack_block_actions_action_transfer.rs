use serde::{Serialize, Deserialize};
use crate::slash_command::transfer::SlackTextTransfer;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsActionTransfer {
    pub action_id: String,
    #[serde(rename = "type")]
    pub action_type: String,
    pub block_id: String,
    pub value: Option<String>,
    pub text: Option<SlackTextTransfer>,
}