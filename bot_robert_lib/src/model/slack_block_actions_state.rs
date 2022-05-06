use crate::model::*;

#[derive(Clone, Debug)]
pub struct SlackBlockActionsState {
    pub name: String,
    pub state_type: SlackElementType,
    pub value: Option<String>,
    pub selected_options: Option<Vec<SlackActionOption>>
}