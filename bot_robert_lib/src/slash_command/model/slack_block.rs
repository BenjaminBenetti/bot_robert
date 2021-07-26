use crate::slash_command::model::slack_block_type::SlackBlockType;
use crate::slash_command::model::slack_element::SlackElement;
use crate::slash_command::model::slack_text::SlackText;
use crate::slash_command::SlackElementType;

#[derive(Clone)]
pub struct SlackBlock {
    pub block_type: SlackBlockType,
    pub elements: Vec<SlackElement>,
    pub label: Option<SlackText>,
    pub text: Option<SlackText>,
}

impl SlackBlock {
    pub fn new(block_type: SlackBlockType, elements: Vec<&SlackElement>, label: Option<&SlackText>, text: Option<&SlackText>) -> SlackBlock {
        SlackBlock {
            block_type,
            elements: elements.into_iter().map(|el| el.clone()).collect(),
            label: label.cloned(),
            text: text.cloned(),
        }
    }

    /// create a slack block that is a text-input
    /// ### params
    /// - label. The label of the text input
    /// - placeholder. Placeholder text to show inside the input
    pub fn new_text_input(label: &str, placeholder: &str) -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Input,
            vec!(&SlackElement::new(
                SlackElementType::PlaneTextInput,
                None,
                None,
                Some(&String::from(placeholder)),
                Some(&String::from("plain_text_input-action"))
            )),
            Some(&SlackText::new(label)),
            None)
    }

    /// create a slack block that is a button
    /// ### params
    /// - text. The text on the button.
    /// - value. The value submitted when the button is clicked.
    /// - action_id. the slack action id for this button.
    pub fn new_button(text: &str, value: &str, action_id: &str) -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Actions,
            vec!(&SlackElement::new(
                SlackElementType::Button,
                Some(&SlackText::new(text)),
                Some(&String::from(value)),
                None,
                Some(&String::from(action_id))
            )),
            None,
            None)
    }

    /// create a slack block that contains text
    /// ### params
    /// - text. The text to display in the block.
    pub fn new_text_block(text: &str) -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Section,
            vec!(),
            None,
            Some(&SlackText::new(text)))
    }
}