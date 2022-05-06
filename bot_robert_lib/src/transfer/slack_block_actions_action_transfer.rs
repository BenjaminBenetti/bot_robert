use serde::{Serialize, Deserialize};
use crate::transfer::{SlackTextTransfer, SlackActionOptionTransfer};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsActionTransfer {
    pub action_id: String,
    #[serde(rename = "type")]
    pub action_type: String,
    pub block_id: String,
    pub value: Option<String>,
    pub text: Option<SlackTextTransfer>,
    pub selected_options: Option<Vec<SlackActionOptionTransfer>>,
}