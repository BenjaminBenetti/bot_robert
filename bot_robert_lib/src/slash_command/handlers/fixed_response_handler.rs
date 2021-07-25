use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::SlackResponse;
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;

pub struct FixedResponseHandler {
    pub command_name: String,
    pub response: SlackResponse,
}

impl FixedResponseHandler {
    pub fn new(command_name: &String, response: &SlackResponse) -> FixedResponseHandler {
        FixedResponseHandler {command_name: command_name.clone(), response: response.clone()}
    }
}

impl CommandProcessor for FixedResponseHandler {
    fn handle_command(&self, _command: &String, _user_name: &String) -> SlackResponse {
        self.response.clone()
    }
}

impl CommandMatcher for FixedResponseHandler {
    fn get_command_name(&self) -> &String {
        &self.command_name
    }
}

impl CommandHandler for FixedResponseHandler {}