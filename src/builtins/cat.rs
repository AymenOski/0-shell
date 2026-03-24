use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Cat;

impl Command for Cat {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement cat command")
    }
    
    fn name() -> &'static str {
        "cat"
    }
    
    fn help() -> &'static str {
        "cat: display file contents"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        !_args.is_empty()
    }
}
