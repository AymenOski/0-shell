use crate::CommandError;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Command { name, args }
    }
}

pub fn parse(input: &str) -> Result<Command, CommandError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(CommandError::InvalidArgs("Empty command".to_string()));
    }
    
    let name = parts[0].to_string();
    let args = parts[1..].iter().map(|s| s.to_string()).collect();
    
    Ok(Command::new(name, args))
}
