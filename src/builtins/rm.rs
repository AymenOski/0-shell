use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Rm;

impl Command for Rm {
    fn execute(args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        if args.is_empty() {
            return Err(CommandError::InvalidArgs("missing operand".to_string()));
        }
        unimplemented!("Implement rm command")
    }
    
    fn name() -> &'static str {
        "rm"
    }
    
    fn help() -> &'static str {
        "rm: remove files or directories"
    }
}
