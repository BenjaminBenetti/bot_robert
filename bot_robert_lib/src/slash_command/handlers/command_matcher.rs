use lazy_regex::regex;

pub trait CommandMatcher {

    fn can_handle_command(&self, command: &String) -> bool {
        let command_regex = regex!(r"^\s*([\w\-\d]+)(\s|$)");

        let captures_opt = command_regex.captures(command);
        if let Some(captures) = captures_opt {
            if let Some(first_cap) = captures.get(1) {
                return first_cap.as_str() == self.get_command_name()
            }
        }

        false
    }

    fn get_command_name(&self) -> &String;
}