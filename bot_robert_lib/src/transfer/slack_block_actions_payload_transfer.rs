use serde::{Serialize, Deserialize};
use crate::transfer::{SlackBlockActionsActionTransfer, SlackBlockActionsStatesTransfer};
use crate::transfer::slack_block_actions_user_transfer::SlackBlockActionsUserTransfer;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsPayloadTransfer {
    pub response_url: String,
    pub state: SlackBlockActionsStatesTransfer,
    pub actions: Vec<SlackBlockActionsActionTransfer>,
    pub user: SlackBlockActionsUserTransfer,
}