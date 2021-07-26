use crate::slash_command::handlers::fixed_response_handler::FixedResponseHandler;
use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::slash_command::SlackResponse;

pub struct JokeAddHandler {
    fixed_response_handler: FixedResponseHandler
}

impl JokeAddHandler {
    pub fn new(command: &String) -> JokeAddHandler {
        JokeAddHandler {
            fixed_response_handler: FixedResponseHandler::new(
                command,
                &SlackResponse::from_string("foobar")
            )
        }
    }
}

impl CommandProcessor for JokeAddHandler {
    fn handle_command(&self, command: &String, user_name: &String) -> SlackResponse {
        self.fixed_response_handler.handle_command(command, user_name)
    }
}

impl CommandMatcher for JokeAddHandler {
    fn get_command_name(&self) -> &String {
        self.fixed_response_handler.get_command_name()
    }
}

impl CommandHandler for JokeAddHandler {}