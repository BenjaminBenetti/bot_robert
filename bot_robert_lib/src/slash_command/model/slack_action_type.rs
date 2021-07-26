
#[derive(Clone, Debug)]
pub enum SlackActionType {
    Button,
}

impl ToString for SlackActionType {
    fn to_string(&self) -> String {
        match self {
            SlackActionType::Button => String::from("button")
        }
    }
}

impl From<String> for SlackActionType {
    fn from(str: String) -> Self {
        return if str == SlackActionType::Button.to_string() {
            SlackActionType::Button
        }
        else {
            SlackActionType::Button
        }
    }
}