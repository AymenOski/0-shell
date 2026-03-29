use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Pwd;

impl Command for Pwd {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        if !args.is_empty() {
            return Err(CommandError::InvalidArgs("too many arguments".to_string()));
        }
        println!("{}", state.current_dir.display());
        Ok(())
    }
    
    fn name() -> &'static str {
        "pwd"
    }
    
    fn help() -> &'static str {
        "pwd: print working directory"
    }
}
