use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SlackInteractionPayloadTransfer {
    #[serde(rename = "type")]
    pub payload_type: String,
}