use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Ls;

impl Command for Ls {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        unimplemented!("Implement ls command")
    }
    
    fn name() -> &'static str {
        "ls"
    }
    
    fn help() -> &'static str {
        "ls: list directory contents"
    }
}
