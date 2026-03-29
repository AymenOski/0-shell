use crate::shell::parser::Command;
use crate::shell::state::ShellState;
use crate::CommandError;
use crate::builtins;
use crate::builtins::Command as CommandTrait;

pub fn dispatch(cmd: Command, state: &mut ShellState) -> Result<(), CommandError> {
    // Convert Vec<String> to Vec<&str> for the execute function
    let args: Vec<&str> = cmd.args.iter().map(|s| s.as_str()).collect();
    
    match cmd.name.as_str() {
        "echo" => builtins::echo::Echo::execute(&args, state),
        "cat" => builtins::cat::Cat::execute(&args, state),
        "cd" => builtins::cd::Cd::execute(&args, state),
        "cp" => builtins::cp::Cp::execute(&args, state),
        "exit" => builtins::exit::Exit::execute(&args, state),
        "ls" => builtins::ls::Ls::execute(&args, state),
        "mkdir" => builtins::mkdir::Mkdir::execute(&args, state),
        "mv" => builtins::mv::Mv::execute(&args, state),
        "pwd" => builtins::pwd::Pwd::execute(&args, state),
        "rm" => builtins::rm::Rm::execute(&args, state),
        _ => Err(CommandError::CommandNotFound(cmd.name.clone())),
    }
}
