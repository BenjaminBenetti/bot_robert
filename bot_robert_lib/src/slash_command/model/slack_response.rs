use crate::slash_command::SlackResponseType;
use crate::slash_command::model::slack_block::SlackBlock;

#[derive(Clone)]
pub struct SlackResponse {
    pub response_type: SlackResponseType,
    pub blocks: Option<Vec<SlackBlock>>,
    pub text: Option<String>,
}

impl SlackResponse {
    pub fn new(response_type: &SlackResponseType, blocks: Option<&Vec<SlackBlock>>, text: Option<&String>) -> SlackResponse {
        SlackResponse {
            response_type: *response_type,
            blocks: blocks.map(|b| b.clone()),
            text: text.map(|t| t.clone()),
        }
    }

    pub fn from_string(str: &String) -> SlackResponse {
        SlackResponse::new(&SlackResponseType::InChannel, None, Some(str))
    }
}