use crate::slash_command::model::slack_element_type::SlackElementType;
use std::collections::HashMap;

#[derive(Clone)]
pub struct SlackElement {
    pub element_type: SlackElementType,
    pub attributes: HashMap<String, String>,
}