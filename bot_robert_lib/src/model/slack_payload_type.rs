
pub enum SlackPayloadType {
    BlockActions,
}

impl ToString for SlackPayloadType {
    fn to_string(&self) -> String {
        match self {
            SlackPayloadType::BlockActions => String::from("block_actions")
        }
    }
}