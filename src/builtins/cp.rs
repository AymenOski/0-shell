use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Cp;

impl Command for Cp {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement cp command")
    }
    
    fn name() -> &'static str {
        "cp"
    }
    
    fn help() -> &'static str {
        "cp: copy files or directories"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        _args.len() >= 2
    }
}
