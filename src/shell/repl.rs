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
        std::io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        // Parse the input
        match parser::parse(input) {
            Ok(cmd) => {
                // Dispatch to the right command
                match dispatcher::dispatch(cmd, &mut state) {
                    Ok(_) => {},
                    Err(e) => println!("{}Error: {}{}", red, e, reset),
                }
            }
            Err(e) => println!("{}Error: {}{}", red, e, reset),
        }
    }
}
