use crate::model::*;

// Yes I know this is becoming a sort of "every thing" struct.... TODO fix
#[derive(Clone)]
pub struct SlackElement {
    pub element_type: SlackElementType,
    pub text: Option<SlackText>,
    pub value: Option<String>,
    pub placeholder: Option<SlackText>,
    pub options: Option<Vec<SlackElement>>,
    pub initial_options: Option<Vec<SlackElement>>,
    pub action_id: Option<String>,
}

impl SlackElement {
    pub fn new(element_type: SlackElementType, text: Option<&SlackText>, value: Option<&String>, placeholder: Option<&String>, action_id: Option<&String>) -> SlackElement {
        SlackElement {
            element_type,
            text: text.cloned(),
            value: value.cloned(),
            placeholder: placeholder.map(|placeholder| SlackText::new_plain_text(placeholder)),
            options: None,
            initial_options: None,
            action_id: action_id.cloned(),
        }
    }

    /// create a new slack checkbox element.
    /// ### params
    /// action_id - the action to trigger on check.
    /// options - A list of checkbox options.
    /// initial_options - A list of checkbox options that indicate which checkboxes should be initially selected
    pub fn new_checkbox(action_id: &String, options: Vec<&SlackElement>, initial_options: Option<Vec<&SlackElement>>) -> SlackElement {
        SlackElement {
            element_type: SlackElementType::Checkboxes,
            text: None,
            value: None,
            placeholder: None,
            options: Some(options.iter().map(|opt| (*opt).clone()).collect()),
            initial_options: initial_options.map(|init| init.iter().map(|opt| (*opt).clone()).collect()),
            action_id: Some(action_id.clone()),
        }
    }

    /// create a new slack checkbox option element
    /// ### params
    /// text - text to show on the option
    /// value - the value of the option (used when receiving the response)
    pub fn new_checkbox_option(text: &SlackText, value: &String) -> SlackElement {
        SlackElement {
            element_type: SlackElementType::NoType,
            text: Some(text.clone()),
            value: Some(value.clone()),
            placeholder: None,
            options: None,
            initial_options: None,
            action_id: None,
        }
    }
}