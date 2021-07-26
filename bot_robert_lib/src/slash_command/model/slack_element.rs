use crate::slash_command::model::slack_element_type::SlackElementType;
use crate::slash_command::SlackText;

#[derive(Clone)]
pub struct SlackElement {
    pub element_type: SlackElementType,
    pub text: Option<SlackText>,
    pub value: Option<String>,
    pub placeholder: Option<SlackText>,
    pub action_id: Option<String>,
}

impl SlackElement {
    pub fn new(element_type: SlackElementType, text: Option<&SlackText>, value: Option<&String>, placeholder: Option<&String>, action_id: Option<&String>) -> SlackElement {
        SlackElement {
            element_type,
            text: text.cloned(),
            value: value.cloned(),
            placeholder: placeholder.map(|placeholder| SlackText::new(placeholder)),
            action_id: action_id.cloned(),
        }
    }
}