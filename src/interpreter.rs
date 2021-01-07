use std::io;
use std::io::Write;

use crate::executor;
use crate::lexer;
use crate::lexer::Token;
use crate::parser;

fn getline() -> String {
    let mut guess = String::new();
    print!(":");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}

fn interpret(repl: bool, input_file: &String) {
    let mut input = input_file.clone();
    let mut program = executor::Program::new();
    let mut repl_paused = false;
    // some "unparsed tokens" data structure here
    loop {
        // get text
        if repl {
            input = getline();
        }
        let tokens: Vec<lexer::Token>;
        // lex the input
        match lexer::lex(&input) {
            Ok(tk) => {
                tokens = tk;
            }
            Err(err) => {
                println!("Lexing Error: {:?}", err);
                continue;
            }
        }

        // todo: functionalize this
        if repl {
            // an empty line is a signal to break out of "paused" interpreter state
            if repl_paused && tokens == [Token::EndOfLine, Token::EndOfInput] {
                repl_paused = false;
            } else if !repl_paused {
                for pausable in &[Token::If, Token::For, Token::While, Token::Repeat] {
                    if tokens.contains(pausable) {
                        repl_paused = true;
                    }
                }
            }
        }

        // println!("{:?}", tokens);
        // parse the line. If we can't parse, add to the "unparsed tokens" and continue
        //                 If we can parse, generate the AST and continue
        match parser::parse(&tokens, &mut program) {
            Err(err) => {
                println!("Parse Error: {:?}", err);
                continue;
            }
            _ => (),
        };

        println!("{:#?}", program);

        if repl_paused {
            continue;
        }

        // calculate the AST
        // print the result
        // save the program counter before we run, in case we need to rewind it
        let pc_backup = program.pc;
        match executor::execute(&mut program) {
            Err(err) => {
                match err {
                    executor::ExecError::UnexpectedEof => (),
                    _ => println!("Execution Error: {:?}", err),
                }
                program.pc = pc_backup;
            }
            _ => {
                // No errors
                if repl {
                    println!("{:?}", program.ctx.ans);
                }
            }
        };

        if !repl {
            break;
        }
    }
}

pub fn interpret_repl() {
    // todo: we will need to trap this to break out of loops eventually
    println!("Ctrl+C to exit, enter twice to execute block of code.\r\n");
    interpret(true, &String::new())
}

pub fn interpret_file(file: &String) {
    interpret(false, file)
}
