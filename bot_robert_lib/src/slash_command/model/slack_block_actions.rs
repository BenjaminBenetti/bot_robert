use crate::slash_command::{SlackBlockActionsAction, SlackBlockActionsState};

#[derive(Clone, Debug)]
pub struct SlackBlockActions {
    pub state: Vec<SlackBlockActionsState>,
    pub response_url: String,
    pub actions: Vec<SlackBlockActionsAction>,
}