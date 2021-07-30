
#[derive(Clone, Debug)]
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

impl From<&String> for SlackTextType {
    fn from(str: &String) -> Self {
        return if *str == SlackTextType::Plain.to_string() {
            SlackTextType::Plain
        }
        else if *str == SlackTextType::MarkDown.to_string() {
            SlackTextType::MarkDown
        }
        else {
            SlackTextType::Plain
        }
    }
}