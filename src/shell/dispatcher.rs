use crate::shell::parser::Command;
use crate::shell::state::ShellState;
use crate::CommandError;
use crate::builtins;
use crate::builtins::Command as CommandTrait;

pub fn dispatch(cmd: Command, state: &mut ShellState) -> Result<(), CommandError> {
    // Convert Vec<String> to Vec<&str> for the execute function
    let args: Vec<&str> = cmd.args.iter().map(|s| s.as_str()).collect();
    
    match cmd.name.as_str() {
        "echo" => {
            if !builtins::echo::Echo::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::echo::Echo::help().to_string()));
            }
            builtins::echo::Echo::execute(&args, state)
        }
        "cat" => {
            if !builtins::cat::Cat::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::cat::Cat::help().to_string()));
            }
            builtins::cat::Cat::execute(&args, state)
        }
        "cd" => {
            if !builtins::cd::Cd::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::cd::Cd::help().to_string()));
            }
            builtins::cd::Cd::execute(&args, state)
        }
        "cp" => {
            if !builtins::cp::Cp::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::cp::Cp::help().to_string()));
            }
            builtins::cp::Cp::execute(&args, state)
        }
        "exit" => {
            if !builtins::exit::Exit::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::exit::Exit::help().to_string()));
            }
            builtins::exit::Exit::execute(&args, state)
        }
        "ls" => {
            if !builtins::ls::Ls::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::ls::Ls::help().to_string()));
            }
            builtins::ls::Ls::execute(&args, state)
        }
        "mkdir" => {
            if !builtins::mkdir::Mkdir::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::mkdir::Mkdir::help().to_string()));
            }
            builtins::mkdir::Mkdir::execute(&args, state)
        }
        "mv" => {
            if !builtins::mv::Mv::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::mv::Mv::help().to_string()));
            }
            builtins::mv::Mv::execute(&args, state)
        }
        "pwd" => {
            if !builtins::pwd::Pwd::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::pwd::Pwd::help().to_string()));
            }
            builtins::pwd::Pwd::execute(&args, state)
        }
        "rm" => {
            if !builtins::rm::Rm::validate_args(&args) {
                return Err(CommandError::InvalidArgs(builtins::rm::Rm::help().to_string()));
            }
            builtins::rm::Rm::execute(&args, state)
        }
        _ => Err(CommandError::InvalidArgs(format!("Command '{}' not found", cmd.name))),
    }
}
