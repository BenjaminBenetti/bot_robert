#[derive(Copy, Clone)]
pub enum SlackElementType {
    PlaneText,
    PlaneTextInput,
    Button,
}

impl ToString for SlackElementType {
    fn to_string(&self) -> String {
        match self {
            SlackElementType::PlaneText => {
                String::from("plain_text")
            }
            SlackElementType::PlaneTextInput => {
                String::from("plain_text_input")
            }
            SlackElementType::Button => {
                String::from("button")
            }
        }
    }
}