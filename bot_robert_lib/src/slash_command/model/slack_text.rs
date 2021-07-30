use crate::slash_command::SlackTextType;

#[derive(Clone, Debug)]
pub struct SlackText {
    pub text_type: SlackTextType,
    pub text: String,
}

impl SlackText {
    pub fn new(text: &str, text_type: SlackTextType) -> SlackText {
        SlackText {
            text_type,
            text: String::from(text)
        }
    }

    pub fn new_plain_text(text: &str) -> SlackText {
        SlackText::new(text, SlackTextType::Plain)
    }

    pub fn new_markdown_text(text: &str) -> SlackText {
        SlackText::new(text, SlackTextType::MarkDown)
    }
}