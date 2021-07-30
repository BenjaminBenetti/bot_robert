use serde::{Serialize, Deserialize};

pub static DB_JOKE_COLLECTION: &str = "jokes";

#[derive(Serialize, Deserialize, Debug)]
pub struct Joke {
    pub setup: String,
    pub punchline: String,
}