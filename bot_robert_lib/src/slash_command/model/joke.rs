use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Joke {
    pub setup: String,
    pub punchline: String,
}