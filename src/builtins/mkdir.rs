use crate::CommandError;
use super::Command;

pub struct Mkdir;

impl Command for Mkdir {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement mkdir command")
    }
    
    fn name() -> &'static str {
        "mkdir"
    }
    
    fn help() -> &'static str {
        "mkdir: create directories"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        !args.is_empty()
    }
}
