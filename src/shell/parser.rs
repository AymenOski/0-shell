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
    let tokens = tokenize(input)?;
    
    if tokens.is_empty() {
        return Err(CommandError::InvalidArgs("Empty command".to_string()));
    }
    
    let name = tokens[0].clone();
    let args = tokens[1..].to_vec();
    
    Ok(Command::new(name, args))
}

/// Check if input has unclosed quotes.
pub fn has_unclosed_quotes(input: &str) -> (bool, Option<char>) {
    match tokenize(input) {
        Ok(_) => (false, None),
        Err(CommandError::InvalidArgs(msg)) => {
            // Extract which quote type failed from the error message
            if msg.contains("double") {
                (true, Some('"'))
            } else if msg.contains("single") {
                (true, Some('\''))
            } else {
                (false, None)
            }
        }
        Err(_) => (false, None),
    }
}

// Tokenize input respecting both single and double quotes.
fn tokenize(input: &str) -> Result<Vec<String>, CommandError> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_double_quote = false;
    let mut in_single_quote = false;
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            // Escape sequence: only in double quotes (single quotes are literal)
            '\\' if in_double_quote => {
                // Peek at next char
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '"' || next_ch == '\\' {
                        // Consume the escaped char after backslash
                        current_token.push(chars.next().unwrap());
                        continue;
                    }
                }
                // Backslash not escaping a quote, add it literally
                current_token.push(ch);
            }
            // Toggle double quote state (only if not in single quotes)
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            // Toggle single quote state (only if not in double quotes)
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            // Whitespace: split token only if not inside quotes
            ' ' | '\t' | '\n' | '\r' if !in_double_quote && !in_single_quote => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            // Regular character: add to current token
            _ => {
                current_token.push(ch);
            }
        }
    }
    
    // Error if quotes weren't closed; it means keep reading lines until they are closed.
    if in_double_quote {
        return Err(CommandError::InvalidArgs(
            "Unclosed double quote".to_string(),
        ));
    }
    if in_single_quote {
        return Err(CommandError::InvalidArgs(
            "Unclosed single quote".to_string(),
        ));
    }
    
    // Add the final token if it's not empty
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    
    Ok(tokens)
}
