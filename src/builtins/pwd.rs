use crate::CommandError;
use super::Command;

pub struct Pwd;

impl Command for Pwd {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement pwd command")
    }
    
    fn name() -> &'static str {
        "pwd"
    }
    
    fn help() -> &'static str {
        "pwd: print working directory"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        true
    }
}
