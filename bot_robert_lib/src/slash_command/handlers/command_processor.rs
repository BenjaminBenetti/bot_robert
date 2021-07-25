use crate::slash_command::SlackResponse;

pub trait CommandProcessor {
    /// handles the provided slack command producing a response
    fn handle_command(&self, command: &String, user_name: &String) -> SlackResponse;
}