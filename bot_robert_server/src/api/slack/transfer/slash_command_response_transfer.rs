use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SlashCommandResponseTransfer {
    pub text: String,
    pub response_type: String,
}

pub enum SlashCommandResponseType {
    InChannel,
    Ephemeral
}

impl SlashCommandResponseType {
    pub fn to_s (&self) -> String {
        match self {
            SlashCommandResponseType::Ephemeral => String::from("ephemeral"),
            SlashCommandResponseType::InChannel => String::from("in_channel"),
        }
    }
}