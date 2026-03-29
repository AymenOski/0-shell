use crate::shell::dispatcher;
use crate::shell::error;
use crate::shell::parser;
use crate::shell::state::ShellState;

pub fn start() {
    let cyan = "\x1b[36m";
    let _green = "\x1b[32m";
    let _yellow = "\x1b[33m";
    let reset = "\x1b[0m";

    println!(
        r"{}
      /$$$$$$                    /$$                 /$$ /$$
     /$$$_  $$                  | $$                | $$| $$
    | $$$$\ $$          /$$$$$$$| $$$$$$$   /$$$$$$ | $$| $$
    | $$ $$ $$ /$$$$$$ /$$_____/| $$__  $$ /$$__  $$| $$| $$
    | $$\ $$$$|______/|  $$$$$$ | $$  \ $$| $$$$$$$$| $$| $$
    | $$ \ $$$         \____  $$| $$  | $$| $$_____/| $$| $$
    |  $$$$$$/         /$$$$$$$/| $$  | $$|  $$$$$$$| $$| $$
     \______/         |_______/ |__/  |__/ \_______/|__/|__/
                                                        
                                                        
                                                        
{}",
        _green, reset
    );
    println!(
        "{}         0-shell  •  minimalist Rust shell{}",
        _green, reset
    );

    // Initialize shell state
    let mut state = match ShellState::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Failed to initialize shell: {}", e);
            return;
        }
    };

    loop {
        print!("{}${} ", cyan, reset);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();

        // Read the first line
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!();
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
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
                    eprintln!("Error: Unexpected EOF in quoted string");
                    input.clear();
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
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
                let cmd_name = cmd.name.clone();
                // Dispatch to the right command
                if let Err(err) = dispatcher::dispatch(cmd, &mut state) {
                    println!("{}", error::format_error(&cmd_name, &err));
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
