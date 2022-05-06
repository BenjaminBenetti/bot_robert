use serde::{Serialize, Deserialize};
use crate::transfer::SlackTextTransfer;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackActionOptionTransfer {
    pub value: String,
    pub text: SlackTextTransfer,
}