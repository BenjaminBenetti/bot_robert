use crate::slash_command::{SlackText};
use crate::slash_command::transfer::SlackTextTransfer;

pub fn convert(slack_text: &SlackText) -> SlackTextTransfer {
    SlackTextTransfer {
        text_type: slack_text.text_type.to_string(),
        text: slack_text.text.clone(),
        emoji: None,
    }
}