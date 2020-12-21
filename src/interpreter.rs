use std::io;
use std::io::Write;

fn getline() -> String {
    let mut guess= String::new();
    print!(":");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}

fn interpret(repl: bool, file: &String) {

    // some "unparsed tokens" data structure here
    loop {
        // get text
        let input = if repl {
            getline();
        } else {
            file;
        };

        // lex the input
        // parse the line. If we can't parse, add to the "unparsed tokens" and continue
        //                 If we can parse, generate the AST and continue
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