use crate::CommandError;
use super::Command;

pub struct Cp;

impl Command for Cp {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement cp command")
    }
    
    fn name() -> &'static str {
        "cp"
    }
    
    fn help() -> &'static str {
        "cp: copy files or directories"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        args.len() >= 2
    }
}
