mod lexer;
mod parser;
mod evaluator;

use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.nabeel>", args[0]);
        return;
    }
    let file_path = &args[1];

    let input = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            return;
        }
    };

    let lexer = Lexer::new(input);
    let mut parser = match Parser::new(lexer) {
        Ok(parser) => parser,
        Err(e) => {
            eprintln!("Error initializing parser: {}", e);
            return;
        }
    };

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            return;
        }
    };
    let mut evaluator = Evaluator::new();
    match evaluator.eval(ast) {
        Ok(_) => (),
        Err(e) => eprintln!("Error evaluating AST: {}", e),
    }
}