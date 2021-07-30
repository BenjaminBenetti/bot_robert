use crate::transfer::SlackElementTransfer;
use crate::model::*;
use crate::converter::*;

pub fn convert(slack_element: &SlackElement) -> SlackElementTransfer {
    SlackElementTransfer {
        element_type: slack_element.element_type.to_string(),
        text: slack_element.text.as_ref().map(|text| slack_text_to_transfer::convert(&text)),
        value: slack_element.value.clone(),
        placeholder: slack_element.placeholder.as_ref().map(|placeholder| slack_text_to_transfer::convert(placeholder)),
        action_id: slack_element.action_id.clone(),
    }
}