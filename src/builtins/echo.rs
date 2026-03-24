use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Echo;

impl Command for Echo {
    fn execute(args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        // Join all arguments with spaces and print them
        if args.is_empty() {
            println!();
        } else {
            println!("{}", args.join(" "));
        }
        Ok(())
    }
    
    fn name() -> &'static str {
        "echo"
    }
    
    fn help() -> &'static str {
        "echo: print text to stdout"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        // echo never fails due to arguments - it accepts 0 or more
        true
    }
}
