
pub enum Command {
    Help,
    TellJoke,
    AddJoke,
    SourceLink,
    DarkMatter,
    Sarcasm,
    NewProject,
    Lunch,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        String::from(match self {
            Command::Help => "help",
            Command::TellJoke => "joke",
            Command::AddJoke => "joke-add",
            Command::SourceLink => "source",
            Command::DarkMatter => "dark-matter",
            Command::Sarcasm => "sarcasm",
            Command::NewProject => "new-project",
            Command::Lunch => "lunch",
        })
    }
}

impl Command {
    ///return a list of tuples representing the "important" commands, combined with a description of the command
    pub fn all() -> Vec<(Command, String)> {
        vec!(
            (Command::TellJoke, "RobertBot will tell everyone a funny joke.".to_string()),
            (Command::AddJoke, "Add a joke to BotRoberts joke dictionary. It will have a chance of being returned in response to the 'joke' command.".to_string()),
            (Command::Lunch, "RobertBot will pick a spot for lunch.".to_string()),
            (Command::DarkMatter, "Let Conan know we are out of Dark Matter.".to_string()),
            (Command::SourceLink, "Get a link to the source code of BotRobert!".to_string()),
            (Command::Help, "Print information on all commands.".to_string()),
        )
    }
}