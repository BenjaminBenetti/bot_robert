use crate::model::*;

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
    /// - action_id. Action id of this input
    pub fn new_text_input(label: &str, placeholder: &str, action_id: &str) -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Input,
            vec!(&SlackElement::new(
                SlackElementType::PlaneTextInput,
                None,
                None,
                Some(&String::from(placeholder)),
                Some(&String::from(action_id))
            )),
            Some(&SlackText::new_plain_text(label)),
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
                Some(&SlackText::new_plain_text(text)),
                Some(&String::from(value)),
                None,
                Some(&String::from(action_id))
            )),
            None,
            None)
    }

    /// create a slack block that contains one or more buttons in a row
    /// ### params
    /// - button_definitions - tuple with three values in the following order
    ///     - button text
    ///     - button value
    ///     - button action id
    /// ### return
    /// - slack block - the newly constructed slack block
    pub fn new_buttons(button_definitions: Vec<(&str, &str, &str)>) -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Actions,
            button_definitions.iter().map(|button_def| {
                SlackElement::new(
                    SlackElementType::Button,
                    Some(&SlackText::new_plain_text(button_def.0)),
                    Some(&String::from(button_def.1)),
                    None,
                    Some(&String::from(button_def.2))
                )
            }).collect::<Vec<SlackElement>>().iter().collect(),
            None,
            None)
    }

    /// create a slack block that is a button. Includes a second button for canceling
    /// ### params
    /// - text. The text on the button.
    /// - value. The value submitted when the button is clicked.
    /// - action_id. the slack action id for this button.
    pub fn new_button_with_cancel(text: &str, value: &str, action_id: &str) -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Actions,
            vec!(&SlackElement::new(
                SlackElementType::Button,
                Some(&SlackText::new_plain_text(text)),
                Some(&String::from(value)),
                None,
                Some(&String::from(action_id))),
                 &SlackElement::new(
                     SlackElementType::Button,
                     Some(&SlackText::new_plain_text("Cancel")),
                     Some(&String::from("cancel")),
                     None,
                     Some(&String::from("cancel-action")))
            ),
            None,
            None)
    }

    /// create a new checkbox slack block
    /// ### params
    /// label - label text to show above the checkboxes
    /// action_id - the id of the checkbox action
    /// options - tuple Label, Value, checked.
    pub fn new_checkbox_block(label: &str, action_id: &String, options: Vec<(String, String, bool)>) -> SlackBlock {
        let option_elements: Vec<SlackElement> = options.iter()
            .map(|opt| SlackElement::new_checkbox_option(&SlackText::new_plain_text(&opt.0), &opt.1)).collect();

        let initial_option_elements: Vec<SlackElement> = options.iter()
            .filter(|opt| opt.2)
            .map(|opt| SlackElement::new_checkbox_option(&SlackText::new_plain_text(&opt.0), &opt.1)).collect();

        SlackBlock::new(
            SlackBlockType::Input,
            vec!(
                &SlackElement::new_checkbox(action_id,
                                            option_elements.iter().collect(),
                                            if initial_option_elements.len() > 0 {
                                                Some(initial_option_elements.iter().collect())
                                            } else {
                                                None
                                            })
            ),
            Some(&SlackText::new_plain_text(&label)),
            None
        )
    }

    /// create a slack block that contains text
    /// ### params
    /// - text. The text to display in the block.
    pub fn new_text_block(text: &str) -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Section,
            vec!(),
            None,
            Some(&SlackText::new_markdown_text(text)))
    }

    /// create a slack block that is a divider
    pub fn new_divider() -> SlackBlock {
        SlackBlock::new(
            SlackBlockType::Divider,
            vec!(),
            None,
            None)
    }
}