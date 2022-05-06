use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::model::*;
use crate::slash_command::handlers::command_matcher::CommandMatcher;
use crate::slash_command::handlers::command_handler::CommandHandler;
use crate::database::DATABASE_CONNECTION;
use std::error::Error;
use async_trait::async_trait;
use mongodb::bson::doc;
use crate::error::DatabaseError;

pub struct JokeAddHandler {
}

impl JokeAddHandler {
    pub fn new() -> JokeAddHandler {
        JokeAddHandler {}
    }


    async fn add_joke_to_db(&self, setup: &str, punchline: &str) -> Result<(), Box<dyn Error + Send>> {
        let db_con = DATABASE_CONNECTION.lock().await;

        let thing = db_con.get_collection(DB_JOKE_COLLECTION)?;
        match thing.insert_one(doc! { "setup": setup, "punchline": punchline}, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(DatabaseError::new(&e.to_string())))
        }
    }
}

#[async_trait]
impl CommandProcessor for JokeAddHandler {
    async fn handle_block_actions_command(&self, block_action: &SlackBlockActions) -> Option<SlackResponse> {
        let filled_states: Vec<&SlackBlockActionsState> = (&block_action.state).into_iter().filter(|state| state.value.is_some()).collect();
        let setup = (&filled_states).into_iter().find(|state| state.name == "setup");
        let punchline = (&filled_states).into_iter().find(|state| state.name == "punchline");

        return if let (Some(setup), Some(punchline)) = (setup, punchline) {
            match self.add_joke_to_db(setup.value.as_ref().unwrap(), punchline.value.as_ref().unwrap()).await {
                Ok(_) => {
                    Some(SlackResponse::from_string_ephemeral(&format!("Joke Added\n--------------\n{}\n{}",
                                                                       setup.value.as_ref().unwrap_or(&String::from("Wut?")),
                                                                       punchline.value.as_ref().unwrap_or(&String::from("Wut?")))))
                }
                Err(_) => {
                    Some(SlackResponse::from_string("hm... I couldn't insert in to MariaDB... How could this happen? It's web-scale!"))
                }
            }
        }
        else
        {
            Some(SlackResponse::from_string_ephemeral("What are you? A Juno user? You must fill out all the fields!"))
        }
    }
}

impl CommandMatcher for JokeAddHandler {
    fn get_command_name(&self) -> String {
        String::from("joke-add-action")
    }
}

impl CommandHandler for JokeAddHandler {}