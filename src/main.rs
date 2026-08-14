mod ast;
mod environment;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod runtime;
mod token;

use std::env;
use std::fs;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: nxl <file.nxl>");
        std::process::exit(1);
    }

    let filename = &args[1];

    let source = match fs::read_to_string(filename) {
        Ok(source) => source,

        Err(error) => {
            eprintln!(
                "NXL ERROR: could not read '{}': {}",
                filename,
                error
            );
            std::process::exit(1);
        }
    };

    // ---------------------------------------------------------
    // Lexer
    // ---------------------------------------------------------

    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,

        Err(error) => {
            eprintln!("NXL LEXER ERROR: {error}");
            std::process::exit(1);
        }
    };

    // ---------------------------------------------------------
    // Parser
    // ---------------------------------------------------------

    let mut parser = Parser::new(tokens);

    let ast = match parser.parse() {
        Ok(ast) => ast,

        Err(error) => {
            eprintln!("NXL PARSER ERROR: {error}");
            std::process::exit(1);
        }
    };

    // ---------------------------------------------------------
    // Interpreter
    // ---------------------------------------------------------

    let mut interpreter = Interpreter::new();

    if let Err(error) = interpreter.interpret(&ast) {
        eprintln!("NXL RUNTIME ERROR: {error}");
        std::process::exit(1);
    }
}