use crate::transfer::SlackTextTransfer;
use crate::model::*;

pub fn convert(transfer: &SlackTextTransfer) -> SlackText {
    SlackText {
        text_type: SlackTextType::from(&transfer.text_type),
        text: transfer.text.clone(),
    }
}