use crate::CommandError;
use super::Command;

pub struct Exit;

impl Command for Exit {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement exit command")
    }
    
    fn name() -> &'static str {
        "exit"
    }
    
    fn help() -> &'static str {
        "exit: exit the shell"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        true
    }
}
