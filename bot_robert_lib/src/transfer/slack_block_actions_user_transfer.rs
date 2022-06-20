use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsUserTransfer {
    pub username: String,
}