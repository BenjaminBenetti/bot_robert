use crate::*;
use crate::slash_command::model::Joke;
use crate::slash_command::SlackResponse;

/// get the list of jokes as slack responses
pub fn jokes_as_slack_responses() -> Vec<SlackResponse> {

    if let Ok(jokes) = serde_json::from_str::<Vec<Joke>>(resources::DAD_JOKES) {
        jokes.into_iter().map(|joke| SlackResponse::from_string(&format!("{}\n{}", joke.setup, joke.punchline))).collect()
    }
    else {
        vec!(SlackResponse::from_string(&String::from("RIP JSON")))
    }
}