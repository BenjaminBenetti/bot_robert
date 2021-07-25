use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SlackResponseTransfer {
    pub text: Option<String>,
    pub response_type: String,
}