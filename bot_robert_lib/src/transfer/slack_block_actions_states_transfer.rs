use serde::{Serialize, Deserialize};
use crate::transfer::SlackBlockActionsStateTransfer;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsStatesTransfer {
    pub values: HashMap<String, SlackBlockActionsStateTransfer>
}