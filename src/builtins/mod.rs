use crate::CommandError;

// Blueprint - contract for all builtin commands
pub trait Command {
    fn execute(args: &[&str]) -> Result<(), CommandError>;
    
    fn name() -> &'static str;
    fn help() -> &'static str;
    fn validate_args(args: &[&str]) -> bool;
}

// Implement all commands in these modules
pub mod echo;
pub mod cat;
pub mod cd;
pub mod cp;
pub mod exit;
pub mod ls;
pub mod mkdir;
pub mod mv;
pub mod pwd;
pub mod rm;
