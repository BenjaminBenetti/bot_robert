use regex::Regex;


pub trait CommandMatcher {

    fn can_handle_command(&self, command: &String) -> bool {
        let command_regex = Regex::new(r"^\s*([\w\d]+)*");

        if let Ok(command_regex) = command_regex {
            let captures_opt = command_regex.captures(command);
            if let Some(captures) = captures_opt {
                if let Some(first_cap) = captures.get(1) {
                    return first_cap.as_str() == self.get_command_name()
                }
            }
        }

        false
    }

    fn get_command_name(&self) -> &String;
}