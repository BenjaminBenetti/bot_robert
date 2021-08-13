use crate::model::*;
use crate::slash_command::jokes;
use crate::slash_command::handlers::fixed_response_handler::FixedResponseHandler;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::slash_command::handlers::simple_random_response_handler::SimpleRandomResponseHandler;
use crate::slash_command::factory::slack_response_factory;
use crate::slash_command::factory::lunch_options_factory;
use crate::slash_command::model::Command;

/// process an incoming slash command
/// ### params
/// `user_name` the user initiating the command
/// `args` the command args string
/// ### return
/// a slack response representing the response to send to the user
pub async fn process_command(user_name: &String, args: &String) -> SlackResponse {
    let command_processors = get_command_processors().await;

    // handle command
    for processor in command_processors {
        if processor.can_handle_command(args) {
            println!("Processing Command: [{}] From User: [{}]", args, user_name);
            return processor.handle_command(args, user_name).unwrap_or(SlackResponse::from_string("Ahhhhhh... I need a beer."))
        }
    }

    return SlackResponse::from_string(&String::from("I dont understand! This is an HR violation!"))
}

/// get command processor list
async fn get_command_processors() -> Vec<Box<dyn CommandHandler>> {
    vec!(
        Box::new(SimpleRandomResponseHandler::new(&Command::Lunch.to_string(), &lunch_options_factory::lunch_options())),
        Box::new(FixedResponseHandler::new(&Command::NewProject.to_string(), &SlackResponse::from_string("No Time"))),
        Box::new(SimpleRandomResponseHandler::new(&Command::Sarcasm.to_string(), &vec!(
            SlackResponse::from_string("I don't understand"),
            SlackResponse::from_string("You're just bad at sarcasm")
        ))),
        Box::new(FixedResponseHandler::new(&Command::DarkMatter.to_string(),
                                           &SlackResponse::from_string("<@U0C0GMADR> we out"))),
        Box::new(FixedResponseHandler::new(&Command::Help.to_string(), &slack_response_factory::help_response())),
        Box::new(FixedResponseHandler::new(&String::from("source"),
                                  &SlackResponse::from_string(&String::from("https://github.com/CanadianCommander/bot_robert")))),
        Box::new(SimpleRandomResponseHandler::new(&String::from("joke"), &jokes::jokes_as_slack_responses().await)),
        Box::new(FixedResponseHandler::new(&String::from("joke-add"), &slack_response_factory::joke_add_response()))
    )
}