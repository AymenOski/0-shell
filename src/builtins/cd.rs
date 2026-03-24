use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Cd;

impl Command for Cd {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement cd command")
    }
    
    fn name() -> &'static str {
        "cd"
    }
    
    fn help() -> &'static str {
        "cd: change directory"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        !_args.is_empty()
    }
}
