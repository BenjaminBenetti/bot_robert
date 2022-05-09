use async_trait::async_trait;
use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::model::{SlackResponse, SlackBlockActions};

pub const ACTION_ID: &str = "nop-action";

pub struct NopResponseHandler {
    pub command_name: String,
}

impl NopResponseHandler {
    pub fn new(command_name: &String) -> NopResponseHandler {
        NopResponseHandler {command_name: command_name.clone()}
    }
}

#[async_trait]
impl CommandProcessor for NopResponseHandler {
    async fn handle_command(&self, _command: &String, _user_name: &String) -> Option<SlackResponse> {
        None
    }

    async fn handle_block_actions_command(&self, _block_actions: &SlackBlockActions) -> Option<SlackResponse> {
        None
    }
}

impl CommandMatcher for NopResponseHandler {
    fn get_command_name(&self) -> String {
        self.command_name.clone()
    }
}

impl CommandHandler for NopResponseHandler {}