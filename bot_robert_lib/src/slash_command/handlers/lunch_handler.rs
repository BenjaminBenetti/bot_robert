use async_trait::async_trait;
use crate::database::DATABASE_CONNECTION;
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::model::{SlackResponse, LunchSpot, LUNCH_SPOT_COLLECTION, BASE_LUNCH_SPOT_SCORE};
use std::error::Error;
use rand::Rng;
use crate::error::{StandardError, DatabaseError};
use futures::StreamExt;
use crate::slash_command::model::Command;

pub struct LunchHandler {
}

impl LunchHandler {
    pub fn new() -> LunchHandler {
        LunchHandler {}
    }

    async fn get_random_lunch_spot(&self) -> Result<LunchSpot, Box<dyn Error + Send>> {
        let db_con = DATABASE_CONNECTION.lock().await;

        let lunch_collection = db_con.get_collection::<LunchSpot>(LUNCH_SPOT_COLLECTION)?;

        match lunch_collection.find(None, None).await {
            Ok(lunch_results) => {
                let mut lunch_spot_options: Vec<Option<LunchSpot>> = lunch_results.map(|result| {
                    match result {
                        Ok(res) => Some(res),
                        Err(_) => None,
                    }
                }).collect().await;

                // filter out closed options
                lunch_spot_options = lunch_spot_options.iter().filter(|spot_opt| {
                    if let Some(spot) = spot_opt {
                        return spot.is_open();
                    }
                    return false;
                }).cloned().collect();

                // combine lunch spots and their scores
                let mut lunch_spot_score: Vec<(f64, LunchSpot)> = lunch_spot_options.iter().filter_map(|spot_opt| {
                    match spot_opt {
                        Some(spot) => Some((f64::max(spot.up_voters.len() as f64 - spot.down_voters.len() as f64 + BASE_LUNCH_SPOT_SCORE, 1f64), spot.clone())),
                        None => None
                    }
                }).collect();

                // normalize scores
                let total = lunch_spot_score.iter().fold(0f64, |total, spot| total + spot.0);
                lunch_spot_score = lunch_spot_score.iter().map(|spot| (spot.0 / total, spot.1.clone())).collect();

                // randomly select lunch spot
                let rand_val = rand::thread_rng().gen::<f64>();
                let lunch_spot: (f64, Option<LunchSpot>) = lunch_spot_score.iter().fold((0f64, None), |select_spot, spot| {
                    if rand_val >= select_spot.0 && rand_val < (select_spot.0 + spot.0) {
                        return (select_spot.0 + spot.0, Some(spot.1.clone()))
                    }
                    return (select_spot.0 + spot.0, select_spot.1)
                });

                if let Some(spot) = lunch_spot.1 {
                    return Ok(spot);
                }
                Err(Box::new(StandardError::new("Some thing wrong..... or no lunch spots.... That's even worse! freak out!")))
            }
            Err(err) => Err(Box::new(DatabaseError::new(&err.to_string())))
        }
    }
}

#[async_trait]
impl CommandProcessor for LunchHandler {
    async fn handle_command(&self, _command: &String, _user_name: &String) -> Option<SlackResponse> {
        match self.get_random_lunch_spot().await {
            Ok(lunch_spot) => Some(SlackResponse::from_string(lunch_spot.name.as_str())),
            Err(e) => {
                println!("Error: {}", e.to_string());
                Some(SlackResponse::from_string("ahhhhhh hmmm..... nothing."))
            },
        }
    }
}

impl CommandMatcher for LunchHandler {
    fn get_command_name(&self) -> String {Command::Lunch.to_string()}
}

impl CommandHandler for LunchHandler {}