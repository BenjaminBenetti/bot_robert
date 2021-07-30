use crate::slash_command::transfer::SlackTextTransfer;
use crate::slash_command::{SlackText, SlackTextType};

pub fn convert(transfer: &SlackTextTransfer) -> SlackText {
    SlackText {
        text_type: SlackTextType::from(&transfer.text_type),
        text: transfer.text.clone(),
    }
}