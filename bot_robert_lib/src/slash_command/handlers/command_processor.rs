use crate::slash_command::{SlackResponse, SlackBlockActions};
use async_trait::async_trait;

#[async_trait]
pub trait CommandProcessor: Sync {
    /// handles the provided slack command producing a response
    fn handle_command(&self, _command: &String, _user_name: &String) -> Option<SlackResponse> { None }
    /// handle the provided slack block command, producing a slack response to send back to slack
    async fn handle_block_actions_command(&self, _block_actions: &SlackBlockActions) -> Option<SlackResponse> { None }
}