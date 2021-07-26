use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::{SlackResponse, SlackBlockActions, SlackBlockActionsState};
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;

pub struct JokeAddHandler {
}

impl JokeAddHandler {
    pub fn new() -> JokeAddHandler {
        JokeAddHandler {}
    }
}

impl CommandProcessor for JokeAddHandler {
    fn handle_block_actions_command(&self, block_action: &SlackBlockActions) -> Option<SlackResponse> {
        let filled_states: Vec<&SlackBlockActionsState> = (&block_action.state).into_iter().filter(|state| state.value.is_some()).collect();
        let setup = (&filled_states).into_iter().find(|state| state.name == "setup");
        let punchline = (&filled_states).into_iter().find(|state| state.name == "punchline");

        return if let (Some(setup), Some(punchline)) = (setup, punchline) {
            Some(SlackResponse::from_string_ephemeral(&format!("Joke Added\n--------------\n{}\n{}",
                                                               setup.value.as_ref().unwrap_or(&String::from("Wut?")),
                                                               punchline.value.as_ref().unwrap_or(&String::from("Wut?")))))
        }
        else
        {
            Some(SlackResponse::from_string_ephemeral("What are you? A Juno user? You must fill out all the fields!"))
        }

    }
}

impl CommandMatcher for JokeAddHandler {
    fn get_command_name(&self) -> String {
        String::from("joke-add-action")
    }
}

impl CommandHandler for JokeAddHandler {}