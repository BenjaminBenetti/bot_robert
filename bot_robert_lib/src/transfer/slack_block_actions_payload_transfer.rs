use serde::{Serialize, Deserialize};
use crate::transfer::{SlackBlockActionsActionTransfer, SlackBlockActionsStatesTransfer};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsPayloadTransfer {
    pub response_url: String,
    pub state: SlackBlockActionsStatesTransfer,
    pub actions: Vec<SlackBlockActionsActionTransfer>,
}