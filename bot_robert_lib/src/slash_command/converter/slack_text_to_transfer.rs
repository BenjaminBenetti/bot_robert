use crate::slash_command::{SlackText, SlackTextType};
use crate::slash_command::transfer::SlackTextTransfer;

pub fn convert(slack_text: &SlackText) -> SlackTextTransfer {
    SlackTextTransfer {
        text_type: SlackTextType::MarkDown.to_string(),
        text: slack_text.text.clone(),
        emoji: None,
    }
}