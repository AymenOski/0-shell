use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Mkdir;

impl Command for Mkdir {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement mkdir command")
    }
    
    fn name() -> &'static str {
        "mkdir"
    }
    
    fn help() -> &'static str {
        "mkdir: create directories"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        !_args.is_empty()
    }
}
