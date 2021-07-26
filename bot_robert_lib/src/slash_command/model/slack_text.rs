
#[derive(Clone, Debug)]
pub struct SlackText {
    pub text: String,
}

impl SlackText {
    pub fn  new(text: &str) -> SlackText {
        SlackText {
            text: String::from(text)
        }
    }
}