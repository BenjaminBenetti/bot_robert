#[derive(Copy, Clone)]
pub enum SlackResponseType {
    InChannel,
    Ephemeral
}

impl SlackResponseType {
    pub fn to_s (&self) -> String {
        match self {
            SlackResponseType::Ephemeral => String::from("ephemeral"),
            SlackResponseType::InChannel => String::from("in_channel"),
        }
    }
}