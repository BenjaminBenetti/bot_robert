use crate::model::*;

#[derive(Clone)]
pub struct SlackResponse {
    pub response_type: Option<SlackResponseType>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub text: Option<String>,
}

impl SlackResponse {
    pub fn new(response_type: Option<&SlackResponseType>, blocks: Option<Vec<&SlackBlock>>, text: Option<&String>) -> SlackResponse {
        SlackResponse {
            response_type: response_type.cloned(),
            blocks: blocks.map(|block_vec| block_vec.into_iter().map(|block| block.clone()).collect()),
            text: text.map(|t| t.clone()),
        }
    }

    pub fn from_string(str: &str) -> SlackResponse {
        SlackResponse::new(Some(&SlackResponseType::InChannel), None, Some(&String::from(str)))
    }

    pub fn from_string_ephemeral(str: &str) -> SlackResponse {
        SlackResponse::new(Some(&SlackResponseType::Ephemeral), None, Some(&String::from(str)))
    }
}