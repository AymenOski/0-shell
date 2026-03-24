use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Rm;

impl Command for Rm {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement rm command")
    }
    
    fn name() -> &'static str {
        "rm"
    }
    
    fn help() -> &'static str {
        "rm: remove files or directories"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        !_args.is_empty()
    }
}
