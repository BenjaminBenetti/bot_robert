use bot_robert_lib::slash_command::{SlackResponse};
use crate::api::slack::transfer::SlackResponseTransfer;

/// convert a slack response to transfer
/// ### params
/// slack_response - the slack response to convert
/// ### return
/// slack_response_transfer - a transfer object of the provided slack response
pub fn convert(slack_response: &SlackResponse) -> SlackResponseTransfer {
    SlackResponseTransfer{
        response_type: slack_response.response_type.to_s(),
        text: slack_response.text.as_ref().map(|s| s.clone())
    }
}