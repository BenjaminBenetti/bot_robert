use serde::{Serialize, Deserialize};
use crate::slash_command::transfer::SlackBlockActionsStateTransfer;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackBlockActionsStatesTransfer {
    pub values: HashMap<String, SlackBlockActionsStateTransfer>
}