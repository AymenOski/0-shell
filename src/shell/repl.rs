use crate::shell::parser;
use crate::shell::dispatcher;
use crate::shell::state::ShellState;


pub fn start() {
    let cyan = "\x1b[36m";
    let _green = "\x1b[32m";
    let _yellow = "\x1b[33m";
    let bold = "\x1b[1m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    
    println!("{}{}",bold, cyan);
    println!("  ██████╗         ███████╗██╗  ██╗███████╗██╗     ██╗");
    println!("  ██╔═══╝         ██╔════╝██║  ██║██╔════╝██║     ██║");
    println!("  ██║  ███╗       ███████╗███████║█████╗  ██║     ██║");
    println!("  ██║   ██║       ╚════██║██╔══██║██╔══╝  ██║     ██║");
    println!("  ╚██████╔╝       ███████║██║  ██║███████╗███████╗███████╗");
    println!("   ╚═════╝        ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝");
    println!();
    println!();

    // Initialize shell state
    let mut state = match ShellState::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}Error: Failed to initialize shell: {}{}", red, e, reset);
            return;
        }
    };
    
    loop {
        print!("{}${} ", cyan, reset);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        
        // Read the first line
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => { // EOF (Ctrl+D)
                println!();
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}Error reading input: {}{}", red, e, reset);
                break;
            }
        }
        
        // If input is just whitespace, skip
        if input.trim().is_empty() {
            continue;
        }
        
        // Keep reading lines if quotes are unclosed
        loop {
            let (has_unclosed, quote_char) = parser::has_unclosed_quotes(&input);
            
            if !has_unclosed {
                // Quotes are closed, break and execute
                break;
            }
            
            // Show continuation prompt based on which quote is unclosed
            let prompt_char = match quote_char {
                Some('"') => "dquote",
                Some('\'') => "squote",
                _ => "cont",
            };
            print!("{}{}{} ", cyan, prompt_char, reset);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            
            // Read the next line
            match std::io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF before closing quote
                    println!();
                    eprintln!("{}Error: Unexpected EOF in quoted string{}", red, reset);
                    input.clear();
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}Error reading input: {}{}", red, e, reset);
                    input.clear();
                    break;
                }
            }
        }
        
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue;
        }
        
        // Parse the input
        match parser::parse(trimmed_input) {
            Ok(cmd) => {
                // Dispatch to the right command
                match dispatcher::dispatch(cmd, &mut state) {
                    Ok(_) => {},
                    Err(crate::CommandError::CommandNotFound(name)) => {
                        println!("{}Command '{}' not found{}", red, name, reset);
                    }
                    Err(crate::CommandError::IsADirectory(path)) => {
                        println!("{}cat: {}: Is a directory{}", red, path, reset);
                    }
                    Err(e) => println!("{}Error: {}{}", red, e, reset),
                }
            }
            Err(e) => println!("{}Error: {}{}", red, e, reset),
        }
    }
}
