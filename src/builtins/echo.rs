use crate::CommandError;
use super::Command;

pub struct Echo;

impl Command for Echo {
    fn execute(args: &[&str]) -> Result<(), CommandError> {
        unimplemented!("Implement echo command")
    }
    
    fn name() -> &'static str {
        "echo"
    }
    
    fn help() -> &'static str {
        "echo: print text to stdout"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        !args.is_empty()
    }
}
