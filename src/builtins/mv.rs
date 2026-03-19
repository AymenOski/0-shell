use crate::CommandError;
use super::Command;

pub struct Mv;

impl Command for Mv {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement mv command")
    }
    
    fn name() -> &'static str {
        "mv"
    }
    
    fn help() -> &'static str {
        "mv: move or rename files"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        args.len() >= 2
    }
}
