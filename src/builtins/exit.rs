use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Exit;

impl Command for Exit {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement exit command")
    }
    
    fn name() -> &'static str {
        "exit"
    }
    
    fn help() -> &'static str {
        "exit: exit the shell"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        true
    }
}
