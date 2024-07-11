mod lexer;
mod parser;
mod evaluator;

use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;

fn main() {
    let input = String::from("print 1 + 2 * 3;");
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
    evaluator.eval(ast);
}