use std::io;
use std::io::Write;

use crate::lexer;
use crate::parser;

fn getline() -> String {
    let mut guess= String::new();
    print!(":");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}

fn interpret(repl: bool, input_file: &String) {

    let mut input = input_file.clone();
    let mut program = parser::Program::new();

    // some "unparsed tokens" data structure here
    loop {
        // get text
        if repl {
            input = getline();
        } 

        // lex the input
        let tokens = lexer::lex(&input);
        // parse the line. If we can't parse, add to the "unparsed tokens" and continue
        //                 If we can parse, generate the AST and continue
        parser::parse(&tokens, &mut program);
        // calculate the AST
        // print the result

        if !repl {
            break;
        }
    }
}

pub fn interpret_repl() {
    interpret(true, &String::new())
}

pub fn interpret_file(file: &String) {
    interpret(false, file)
}