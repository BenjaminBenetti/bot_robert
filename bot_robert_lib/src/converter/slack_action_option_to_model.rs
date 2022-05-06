use crate::converter::*;
use crate::transfer::SlackActionOptionTransfer;
use crate::model::SlackActionOption;

pub fn convert(transfer: &SlackActionOptionTransfer) -> SlackActionOption {
    SlackActionOption {
        value: transfer.value.clone(),
        text: slack_text_transfer_to_model::convert(&transfer.text),
    }
}