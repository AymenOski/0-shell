use crate::CommandError;
use super::Command;

pub struct Ls;

impl Command for Ls {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement ls command")
    }
    
    fn name() -> &'static str {
        "ls"
    }
    
    fn help() -> &'static str {
        "ls: list directory contents"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        true
    }
}
