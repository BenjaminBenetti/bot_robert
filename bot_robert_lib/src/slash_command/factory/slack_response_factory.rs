use crate::slash_command::{SlackResponse, SlackBlock, Command, SlackResponseType};

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