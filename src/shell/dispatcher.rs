use crate::shell::parser::Command;
use crate::CommandError;
use crate::builtins;
use crate::builtins::Command as CommandTrait;

pub fn dispatch(cmd: Command) -> Result<(), CommandError> {
    // Convert Vec<String> to Vec<&str> for the execute function
    let args: Vec<&str> = cmd.args.iter().map(|s| s.as_str()).collect();
    
    match cmd.name.as_str() {
        "echo" => builtins::echo::Echo::execute(&args),
        "cat" => builtins::cat::Cat::execute(&args),
        "cd" => builtins::cd::Cd::execute(&args),
        "cp" => builtins::cp::Cp::execute(&args),
        "exit" => builtins::exit::Exit::execute(&args),
        "ls" => builtins::ls::Ls::execute(&args),
        "mkdir" => builtins::mkdir::Mkdir::execute(&args),
        "mv" => builtins::mv::Mv::execute(&args),
        "pwd" => builtins::pwd::Pwd::execute(&args),
        "rm" => builtins::rm::Rm::execute(&args),
        _ => Err(CommandError::InvalidArgs(format!("Unknown command: {}", cmd.name))),
    }
}
