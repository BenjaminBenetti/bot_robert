use crate::slash_command::transfer::SlackBlockActionsActionTransfer;
use crate::slash_command::{SlackBlockActionsAction, SlackActionType};
use crate::slash_command::converter::*;

pub fn convert(transfer: &SlackBlockActionsActionTransfer) -> SlackBlockActionsAction {
    SlackBlockActionsAction {
        action_id: transfer.action_id.clone(),
        action_type: SlackActionType::from(transfer.action_type.clone()),
        block_id: transfer.block_id.clone(),
        value: transfer.value.clone(),
        text: transfer.text.as_ref().map(|text| slack_text_transfer_to_model::convert(text)),
    }
}