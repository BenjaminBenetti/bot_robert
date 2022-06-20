use crate::model::{LunchSpot, SlackResponse, SlackResponseType, SlackBlock};
use crate::slash_command::handlers::lunch_handler::{LUNCH_UP_VOTE_ACTION_ID, LUNCH_DOWN_VOTE_ACTION_ID};

/**
build a lunch spot response for the given lunch spot
*/
pub fn build_lunch_spot_response(lunch_spot: &LunchSpot) -> SlackResponse {
    let mut blocks: Vec<SlackBlock> = vec!(
        SlackBlock::new_text_block(&lunch_spot.name),
        SlackBlock::new_buttons(vec!(
            (":yum: Up Vote", &lunch_spot.name, LUNCH_UP_VOTE_ACTION_ID),
            (":disappointed: Down Vote", &lunch_spot.name, LUNCH_DOWN_VOTE_ACTION_ID),
        )),
    );

    if lunch_spot.up_voters.len() > 0
    {
        blocks.push(SlackBlock::new_text_block(lunch_spot.up_voters.iter().fold(":yum:".to_string(), |acc, voter|  format!("{} {}", acc, voter)).as_ref()));
    }

    if lunch_spot.down_voters.len() > 0
    {
        blocks.push(SlackBlock::new_text_block(lunch_spot.down_voters.iter().fold(":disappointed:".to_string(), |acc, voter|  format!("{} {}", acc,  voter)).as_ref()));
    }

    SlackResponse::new(
        Some(&SlackResponseType::InChannel),
        Some(blocks.iter().collect()),
        None
    )
}