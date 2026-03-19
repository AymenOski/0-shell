use crate::CommandError;
use super::Command;

pub struct Cd;

impl Command for Cd {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement cd command")
    }
    
    fn name() -> &'static str {
        "cd"
    }
    
    fn help() -> &'static str {
        "cd: change directory"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        !args.is_empty()
    }
}
