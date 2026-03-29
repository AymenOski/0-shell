use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Exit;

impl Command for Exit {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        std::process::exit(0);
    }
    
    fn name() -> &'static str {
        "exit"
    }
    
    fn help() -> &'static str {
        "exit: exit the shell"
    }
}
