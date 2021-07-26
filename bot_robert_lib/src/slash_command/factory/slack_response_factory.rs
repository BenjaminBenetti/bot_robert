use crate::slash_command::{SlackResponse, SlackBlock};

/// get a slack response that presents the joke add interface.
/// Users use this to add new jokes to the system.
pub fn joke_add_response() -> SlackResponse {
    SlackResponse::new(
        None,
        Some(vec!(
            // instructions.
            &SlackBlock::new_text_block("Add a new joke to RobertBot!"),
            // Joke Setup input
            &SlackBlock::new_text_input("Setup", "Setup of your joke"),
            // Joke punchline input
            &SlackBlock::new_text_input("Punchline", "Punchline of your joke"),
            // Joke add button. User clicks to add the joke!
            &SlackBlock::new_button("Add Joke", "joke-add", "joke-add-action")
        )),
        None,
    )
}