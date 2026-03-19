use crate::CommandError;
use super::Command;

pub struct Cat;

impl Command for Cat {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement cat command")
    }
    
    fn name() -> &'static str {
        "cat"
    }
    
    fn help() -> &'static str {
        "cat: display file contents"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        !args.is_empty()
    }
}
