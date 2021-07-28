use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::{SlackResponse, SlackBlockActions};
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use async_trait::async_trait;

pub struct FixedResponseHandler {
    pub command_name: String,
    pub response: SlackResponse,
}

impl FixedResponseHandler {
    pub fn new(command_name: &String, response: &SlackResponse) -> FixedResponseHandler {
        FixedResponseHandler {command_name: command_name.clone(), response: response.clone()}
    }
}

#[async_trait]
impl CommandProcessor for FixedResponseHandler {
    fn handle_command(&self, _command: &String, _user_name: &String) -> Option<SlackResponse> {
        Some(self.response.clone())
    }

    async fn handle_block_actions_command(&self, _block_actions: &SlackBlockActions) -> Option<SlackResponse> {
        Some(self.response.clone())
    }
}

impl CommandMatcher for FixedResponseHandler {
    fn get_command_name(&self) -> String {
        self.command_name.clone()
    }
}

impl CommandHandler for FixedResponseHandler {}