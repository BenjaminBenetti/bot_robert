use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SlackFormPayloadTransfer {
    pub payload: String,
}