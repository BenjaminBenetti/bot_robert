use crate::model::SlackText;

#[derive(Clone, Debug)]
pub struct SlackActionOption {
    pub value: String,
    pub text: SlackText
}