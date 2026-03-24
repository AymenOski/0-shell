use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Pwd;

impl Command for Pwd {
    fn execute(_args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        println!("{}", state.current_dir.display());
        Ok(())
    }
    
    fn name() -> &'static str {
        "pwd"
    }
    
    fn help() -> &'static str {
        "pwd: print working directory"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        true
    }
}
