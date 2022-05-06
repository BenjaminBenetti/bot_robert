use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::transfer::SlackActionOptionTransfer;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsStateTransfer {
    #[serde(flatten)]
    pub states: HashMap<String, State>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde(rename = "type")]
    pub state_type: String,
    pub value: Option<String>,
    pub selected_options: Option<Vec<SlackActionOptionTransfer>>
}