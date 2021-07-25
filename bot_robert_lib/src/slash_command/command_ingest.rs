use regex::Regex;
use crate::slash_command::{jokes, SlackResponse};
use crate::slash_command::handlers::fixed_response_handler::FixedResponseHandler;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::slash_command::handlers::simple_random_response_handler::SimpleRandomResponseHandler;

/// process an incoming slash command
/// ### params
/// `user_name` the user initiating the command
/// `args` the command args string
/// ### return
/// a slack response representing the response to send to the user
pub fn process_command(user_name: &String, args: &String) -> SlackResponse {
    let command_processors = get_command_processors();
    let robert_regex = Regex::new(r"^[rR]obert.*");

    if let Ok(robert_reg) = robert_regex {
        if robert_reg.is_match(&user_name) {
            return SlackResponse::from_string(&String::from("I'm talking to my self!"));
        }

        // handle command
        for processor in command_processors {
            if processor.can_handle_command(args) {
                println!("Processing Command: [{}] From User: [{}]", args, user_name);
                return processor.handle_command(args, user_name)
            }
        }

        return SlackResponse::from_string(&String::from("I dont understand! This is an HR violation!"))
    }
    else {
        return SlackResponse::from_string(&String::from("Uh-O ... some this went wrong :("));
    }
}

/// get command processor list
fn get_command_processors() -> Vec<Box<dyn CommandHandler>> {
    vec!(
        Box::new(FixedResponseHandler::new(&String::from("source"),
                                  &SlackResponse::from_string(&String::from("https://github.com/CanadianCommander/bot_robert")))),
        Box::new(SimpleRandomResponseHandler::new(&String::from("joke"), &jokes::jokes_as_slack_responses()))
    )
}