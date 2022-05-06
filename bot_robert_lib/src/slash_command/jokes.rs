use crate::*;
use crate::model::*;
use std::error::Error;
use futures::StreamExt;
use crate::error::DatabaseError;

/// get the list of jokes as slack responses
pub async fn jokes_as_slack_responses() -> Vec<SlackResponse> {

    if let Ok(mut jokes) = serde_json::from_str::<Vec<Joke>>(resources::DAD_JOKES) {
        if let Ok(mut custom_jokes) = get_custom_jokes().await {
            jokes.append(&mut custom_jokes)
        }
        jokes.into_iter().map(|joke| SlackResponse::from_string(&format!("{}\n{}", joke.setup, joke.punchline))).collect()
    }
    else {
        vec!(SlackResponse::from_string(&String::from("RIP JSON")))
    }
}

async fn get_custom_jokes() -> Result<Vec<Joke>, Box<dyn Error + Send>> {
    let db_con = database::DATABASE_CONNECTION.lock().await;

    let joke_collection = db_con.get_collection::<Joke>(DB_JOKE_COLLECTION)?;
    let result = joke_collection.find(None, None).await;

    if let Err(error) = result {
        return Err(Box::new(DatabaseError::new(&error.to_string())));
    }

    let cursor = result.unwrap();
    let jokes: Vec<Option<Joke>> = cursor.map(|document| {
        let document = match document {
            Ok(doc) => doc,
            Err(_) => return None,
        };

        Some(document)
    }).collect().await;

    // In theory both this and the last lines could be combined. futures's (rust crate) streams seem to freak out when chaining map & filter.
    // Probably just me being stupid.
    Ok(jokes.into_iter().filter(|joke| joke.is_some()).map(|joke| joke.unwrap()).collect())
}