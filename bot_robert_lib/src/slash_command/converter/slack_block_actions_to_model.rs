use crate::slash_command::transfer::{SlackBlockActionsPayloadTransfer, SlackBlockActionsActionTransfer};
use crate::slash_command::SlackBlockActions;
use crate::slash_command::converter::*;

pub fn convert(transfer: &SlackBlockActionsPayloadTransfer) -> SlackBlockActions {
    SlackBlockActions {
        state: slack_block_actions_states_to_model::convert(&transfer.state),
        response_url: transfer.response_url.clone(),
        actions: AsRef::<Vec<SlackBlockActionsActionTransfer>>::as_ref(&transfer.actions).into_iter()
            .map(|action| slack_block_actions_action_to_model::convert(action)).collect()
    }
}