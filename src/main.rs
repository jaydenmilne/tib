mod interpreter;
use std::env;
use std::fs;
fn main() {

    let args: Vec<String> = env::args().collect();
    // For now, we'll just do a REPL since that's the tricker thing I want to get working
    if args.len() == 1 {
        println!("tib {} (c) 2020 Jayden Milne", env!("CARGO_PKG_VERSION"));
        interpreter::interpret_repl();
    } else {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(file) => interpreter::interpret_file(&file),
            Err(error) => panic!("Could not open the file {}, error is {:?}", filename, error),
        };
    }
}