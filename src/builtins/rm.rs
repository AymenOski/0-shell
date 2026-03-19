use crate::CommandError;
use super::Command;

pub struct Rm;

impl Command for Rm {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement rm command")
    }
    
    fn name() -> &'static str {
        "rm"
    }
    
    fn help() -> &'static str {
        "rm: remove files or directories"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        !args.is_empty()
    }
}
