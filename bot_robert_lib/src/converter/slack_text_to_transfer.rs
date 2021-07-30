use crate::model::*;
use crate::transfer::SlackTextTransfer;

pub fn convert(slack_text: &SlackText) -> SlackTextTransfer {
    SlackTextTransfer {
        text_type: slack_text.text_type.to_string(),
        text: slack_text.text.clone(),
        emoji: None,
    }
}