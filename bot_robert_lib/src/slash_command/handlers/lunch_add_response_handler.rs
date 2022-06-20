use async_trait::async_trait;
use crate::database::DATABASE_CONNECTION;
use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::model::{SlackBlockActions, SlackResponse, LUNCH_SPOT_COLLECTION, LunchSpot};
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::factory::lunch_spot_factory;
use mongodb::error::{ErrorKind};

pub const ACTION_ID: &str = "lunch_add";

pub struct LunchAddResponseHandler {
}

impl LunchAddResponseHandler {
    pub fn new() -> LunchAddResponseHandler {
        LunchAddResponseHandler {
        }
    }
}

#[async_trait]
impl CommandProcessor for LunchAddResponseHandler {
    async fn handle_block_actions_command(&self, block_actions: &SlackBlockActions) -> Option<SlackResponse> {
        let lunch_spot = lunch_spot_factory::create_lunch_spot_from_form(&block_actions.state);

        match lunch_spot {
            Ok(spot) => {
                let db_con = DATABASE_CONNECTION.lock().await;

                if let Ok(lunch_spot_collection) = db_con.get_collection::<LunchSpot>(LUNCH_SPOT_COLLECTION) {
                    match lunch_spot_collection.insert_one(spot.clone(), None).await {
                        Ok(_) => return Some(SlackResponse::from_string(&format!("I've added \"{}\" to the menu!", spot.name))),
                        Err(err) => {
                            match err.kind.as_ref() {
                                ErrorKind::WriteError(_) => return Some(SlackResponse::from_string("Nice try Brain.")),
                                _ => (),
                            };
                        },
                    }
                }
                return Some(SlackResponse::from_string(&format!(":thinking_face: my webscale DB has failed me again!")))
            }
            Err(e) => return Some(SlackResponse::from_string(&format!(":man-facepalming: {}", e.to_string())))
        }
    }
}

impl CommandMatcher for LunchAddResponseHandler {
    fn get_command_name(&self) -> String {
        ACTION_ID.to_string()
    }
}

impl CommandHandler for LunchAddResponseHandler {}