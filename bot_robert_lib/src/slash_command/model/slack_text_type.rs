
pub enum SlackTextType {
    Plain,
    MarkDown,
}

impl ToString for SlackTextType {
    fn to_string(&self) -> String {
        match self {
            SlackTextType::Plain => String::from("plain_text"),
            SlackTextType::MarkDown => String::from("mrkdwn"),
        }
    }
}