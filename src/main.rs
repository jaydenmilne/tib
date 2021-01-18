mod executor;
mod interpreter;
mod lexer;
mod parser;

use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("tib {} (c) 2020 Jayden Milne", env!("CARGO_PKG_VERSION"));
        interpreter::interpret_repl();
    } else {
        if args[1] == "--help" || args[1] == "-h" {
            println!("tib {} (c) 2020 Jayden Milne", env!("CARGO_PKG_VERSION"));
            println!("Usage: tib [filename, optional]");
            println!("If no filename is provided, you will enter a REPL");
            return;
        }

        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(file) => interpreter::interpret_file(&file),
            Err(error) => panic!("Could not open the file {}, error is {:?}", filename, error),
        };
    }
}
