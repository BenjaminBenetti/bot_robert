use crate::model::*;
use crate::slash_command::model::Command;
use crate::slash_command::handlers::{nop_response_handler, lunch_add_response_handler};
use chrono::Weekday;

/// get a slack response that presents the joke add interface.
/// Users use this to add new jokes to the system.
pub fn joke_add_response() -> SlackResponse {
    SlackResponse::new(
        None,
        Some(vec!(
            // instructions.
            &SlackBlock::new_text_block("Add a new joke to RobertBot!"),
            // Joke Setup input
            &SlackBlock::new_text_input("Setup", "Setup of your joke", "setup"),
            // Joke punchline input
            &SlackBlock::new_text_input("Punchline", "Punchline of your joke", "punchline"),
            // Joke add button. User clicks to add the joke!
            &SlackBlock::new_button_with_cancel("Add Joke", "joke-add", "joke-add-action")
        )),
        None,
    )
}

/// build a lunch add response
/// ### params
/// open-[mon-fri] - is the lunch spot open on the day or not.
pub fn lunch_add_response(open_monday: bool,
                          open_tuesday: bool,
                          open_wednesday: bool,
                          open_thursday: bool,
                          open_friday: bool) -> SlackResponse {
    SlackResponse::new(
        None,
        Some(vec!(
            &SlackBlock::new_text_block("Add a new lunch spot! :hamburger:"),
            // Lunch spot name
            &SlackBlock::new_text_input("Name", "Lunch spot name", "name"),
            // availability
            &SlackBlock::new_checkbox_block("When is it open?", &nop_response_handler::ACTION_ID.to_string(), vec!(
                ("Monday".to_string(), Weekday::Mon.to_string(), open_monday),
                ("Tuesday".to_string(), Weekday::Tue.to_string(), open_tuesday),
                ("Wednesday".to_string(), Weekday::Wed.to_string(), open_wednesday),
                ("Thursday".to_string(), Weekday::Thu.to_string(), open_thursday),
                ("Friday".to_string(), Weekday::Fri.to_string(), open_friday),
            )),
            // Lunch spot add button
            &SlackBlock::new_button_with_cancel("Add lunch spot", "lunch-add", lunch_add_response_handler::ACTION_ID)
        )),
        None
    )
}

/// help message that informs users of RobertBot's features
pub fn help_response() -> SlackResponse {
    let command_text = Command::all().into_iter().fold(String::new(),|command_description, cmd| {
        let (command, description) = cmd;
        format!("{}*{}* - {}\n", command_description, command.to_string(), description)
    });

    SlackResponse::new(Some(&SlackResponseType::Ephemeral), Some(vec!(
        &SlackBlock::new_text_block("*RobertBot the future of Robert.*\n_Usage_: /robert-bot <command> [options]\n"),
        &SlackBlock::new_divider(),
        &SlackBlock::new_text_block(&format!(
        "*= Commands =* \n\
        {}", command_text))
    )), None)
}