use crate::slash_command::{SlackText, SlackActionType};

#[derive(Clone, Debug)]
pub struct SlackBlockActionsAction {
    pub action_id: String,
    pub action_type: SlackActionType,
    pub block_id: String,
    pub value: Option<String>,
    pub text: Option<SlackText>,
}