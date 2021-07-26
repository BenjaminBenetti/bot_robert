use crate::slash_command::SlackElementType;

#[derive(Clone, Debug)]
pub struct SlackBlockActionsState {
    pub name: String,
    pub state_type: SlackElementType,
    pub value: Option<String>,
}