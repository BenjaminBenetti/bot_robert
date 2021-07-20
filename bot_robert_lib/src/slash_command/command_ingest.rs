use regex::Regex;
use crate::slash_command::joke_handler;

/// process an incoming slash command
/// ### params
/// `user_name` the user initiating the command
/// `args` the command args string
/// ### return
/// a string response to send to the user
pub fn process_command(user_name: &String, args: &String) -> String {
    let robert_regex = Regex::new(r"^[rR]obert.*");
    let joke_regex = Regex::new(r"^\s*joke.*");

    if let (Ok(joke_reg), Ok(robert_reg)) = (joke_regex, robert_regex) {
        if robert_reg.is_match(&user_name) {
            String::from("I'm talking to my self!")
        }
        else if joke_reg.is_match(&args) {
            joke_handler::tell_joke(&args)
        }
        else {
            String::from("I dont understand! This is an HR violation!")
        }
    }
    else
    {
        String::from("Uh-O ... some this went wrong :(")
    }
}