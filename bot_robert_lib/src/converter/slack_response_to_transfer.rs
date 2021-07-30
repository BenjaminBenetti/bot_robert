use crate::model::*;
use crate::transfer::SlackResponseTransfer;
use crate::converter::*;

/// convert a slack response to transfer
/// ### params
/// slack_response - the slack response to convert
/// ### return
/// slack_response_transfer - a transfer object of the provided slack response
pub fn convert(slack_response: &SlackResponse) -> SlackResponseTransfer {
    SlackResponseTransfer{
        channel: slack_response.channel.clone(),
        response_type: slack_response.response_type.map(|resp_type| resp_type.to_s()),
        blocks: slack_response.blocks.as_ref().map(|blocks| AsRef::<Vec<SlackBlock>>::as_ref(&blocks).into_iter().map(|block| slack_block_to_transfer::convert(block)).collect()),
        text: slack_response.text.clone(),
    }
}

