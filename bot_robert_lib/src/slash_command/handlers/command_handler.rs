use crate::slash_command::handlers::command_processor::CommandProcessor;
use crate::slash_command::handlers::command_matcher::CommandMatcher;

pub trait CommandHandler: CommandProcessor + CommandMatcher {}