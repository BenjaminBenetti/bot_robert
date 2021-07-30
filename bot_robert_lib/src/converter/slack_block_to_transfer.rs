use crate::model::*;
use crate::transfer::SlackBlockTransfer;
use crate::converter::*;

pub fn convert(slack_block: &SlackBlock) -> SlackBlockTransfer {
    // slack, depending on block type, requires "element" or "elements".
    match slack_block.block_type {
        SlackBlockType::Actions => {
            SlackBlockTransfer {
                block_type: slack_block.block_type.to_s(),
                element: None,
                elements: Some(AsRef::<Vec<SlackElement>>::as_ref(&slack_block.elements).into_iter().map(|el| slack_element_to_transfer::convert(el)).collect()),
                label: slack_block.label.as_ref().map(|label| slack_text_to_transfer::convert(&label)),
                text: slack_block.text.as_ref().map(|text| slack_text_to_transfer::convert(&text)),
            }
        }
        _ => {
            SlackBlockTransfer {
                block_type: slack_block.block_type.to_s(),
                element: slack_block.elements.get(0).map(|el| slack_element_to_transfer::convert(el)),
                elements: None,
                label: slack_block.label.as_ref().map(|label| slack_text_to_transfer::convert(label)),
                text: slack_block.text.as_ref().map(|text| slack_text_to_transfer::convert(&text)),
            }
        }
    }
}