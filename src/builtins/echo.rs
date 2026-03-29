use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;

pub struct Echo;

impl Command for Echo {
    fn execute(args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        if args.is_empty() {
            println!();
        } else {
            let output = args.iter()
                .map(|arg| unescape(arg))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", output);
        }
        Ok(())
    }
    
    fn name() -> &'static str {
        "echo"
    }
    
    fn help() -> &'static str {
        "echo: print text to stdout"
    }
}

// Interpret escape sequences: \n, \t, \\
fn unescape(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next) = chars.peek() {
                match next {
                    'n' => {
                        result.push('\n');
                        chars.next();
                    }
                    't' => {
                        result.push('\t');
                        chars.next();
                    }
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    _ => {
                        result.push(ch);
                    }
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}
