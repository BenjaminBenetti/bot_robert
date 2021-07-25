use crate::slash_command::model::slack_block_type::SlackBlockType;
use crate::slash_command::model::slack_element::SlackElement;
use crate::slash_command::model::slack_label::SlackLabel;

#[derive(Clone)]
pub struct SlackBlock {
    pub block_type: SlackBlockType,
    pub elements: Vec<SlackElement>,
    pub label: SlackLabel,
}