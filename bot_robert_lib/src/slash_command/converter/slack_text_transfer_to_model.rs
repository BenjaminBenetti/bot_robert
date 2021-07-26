use crate::slash_command::transfer::SlackTextTransfer;
use crate::slash_command::SlackText;

pub fn convert(transfer: &SlackTextTransfer) -> SlackText {
    SlackText {
        text: transfer.text.clone(),
    }
}