use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Mv;

impl Command for Mv {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement mv command")
    }
    
    fn name() -> &'static str {
        "mv"
    }
    
    fn help() -> &'static str {
        "mv: move or rename files"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        _args.len() >= 2
    }
}
