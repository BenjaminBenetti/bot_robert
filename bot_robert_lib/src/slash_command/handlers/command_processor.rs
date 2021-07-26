use crate::slash_command::{SlackResponse, SlackBlockActions};

pub trait CommandProcessor {
    /// handles the provided slack command producing a response
    fn handle_command(&self, _command: &String, _user_name: &String) -> Option<SlackResponse> { None }
    /// handle the provided slack block command, producing a slack response to send back to slack
    fn handle_block_actions_command(&self, _block_actions: &SlackBlockActions) -> Option<SlackResponse> { None }
}