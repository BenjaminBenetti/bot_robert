use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SlashCommandTransfer {
    pub channel_id: String, // channel in which slash command was triggered
    pub channel_name: String, // channel name
    pub user_id: String, // user that triggered the slash command
    pub user_name: String, // user name
    pub command: String, // slash command
    pub text: String, // parameters to the slash command
}
