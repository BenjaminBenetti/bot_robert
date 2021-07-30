use crate::model::*;

#[derive(Clone, Debug)]
pub struct SlackBlockActions {
    pub state: Vec<SlackBlockActionsState>,
    pub response_url: String,
    pub actions: Vec<SlackBlockActionsAction>,
}