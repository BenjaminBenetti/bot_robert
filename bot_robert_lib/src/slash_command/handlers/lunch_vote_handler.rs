use async_trait::async_trait;
use crate::database::DATABASE_CONNECTION;
use mongodb::{bson::doc};
use crate::model::{SlackResponse, SlackBlockActions, LUNCH_SPOT_COLLECTION, LunchSpot};
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::handlers::lunch_handler::{LUNCH_UP_VOTE_ACTION_ID, LUNCH_DOWN_VOTE_ACTION_ID};
use crate::slash_command::factory::lunch_spot_response_factory::build_lunch_spot_response;

pub struct LunchVoteHandler {
}

impl LunchVoteHandler {
    pub fn new() -> LunchVoteHandler {
        LunchVoteHandler {}
    }
}

#[async_trait]
impl CommandProcessor for LunchVoteHandler {
    async fn handle_block_actions_command(&self, _block_actions: &SlackBlockActions) -> Option<SlackResponse> {
        let db_con = DATABASE_CONNECTION.lock().await;
        let vote_action_opt = _block_actions.actions
            .iter()
            .find(|action| action.action_id == LUNCH_UP_VOTE_ACTION_ID || action.action_id == LUNCH_DOWN_VOTE_ACTION_ID);


        if let Some(vote_action) = vote_action_opt {
            if let Ok(lunch_spot_collection) = db_con.get_collection::<LunchSpot>(LUNCH_SPOT_COLLECTION)
            {
                if let Ok(Some(mut lunch_spot)) = lunch_spot_collection.find_one(
                    Some(doc! {"name": vote_action.value.as_ref().unwrap_or(&"".to_string())}),
                    None).await
                {
                    if vote_action.action_id == LUNCH_UP_VOTE_ACTION_ID
                    {
                        lunch_spot.up_voters.insert(_block_actions.username.clone());
                        lunch_spot.down_voters.remove(&_block_actions.username);
                    }
                    else if vote_action.action_id == LUNCH_DOWN_VOTE_ACTION_ID
                    {
                        lunch_spot.down_voters.insert(_block_actions.username.clone());
                        lunch_spot.up_voters.remove(&_block_actions.username);
                    }

                    let _ = lunch_spot_collection.replace_one(doc! {"name": vote_action.value.as_ref().unwrap_or(&"".to_string())}, lunch_spot.clone(), None).await;

                    return Some(build_lunch_spot_response(&lunch_spot));
                }
            }
        }

        None
    }
}

impl CommandMatcher for LunchVoteHandler {

    fn get_command_name(&self) -> String { LUNCH_UP_VOTE_ACTION_ID.to_string() }

    fn get_command_names(&self) -> Vec<String> {
        vec![
            LUNCH_UP_VOTE_ACTION_ID.to_string(),
            LUNCH_DOWN_VOTE_ACTION_ID.to_string(),
        ]
    }
}

impl CommandHandler for LunchVoteHandler {}