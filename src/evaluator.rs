use crate::parser::{ASTNode};
use crate::lexer::Token;
use std::collections::HashMap;

pub struct Evaluator {
    variables: HashMap<String, i64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, node: ASTNode) -> i64 {
        match node {
            ASTNode::Number(value) => value,
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    _ => panic!("Unknown operator: {:?}", op),
                }
            }
            ASTNode::Identifier(name) => *self.variables.get(&name).unwrap(),
            ASTNode::Assign(name, expr) => {
                let value = self.eval(*expr);
                self.variables.insert(name, value);
                value
            }
            ASTNode::Print(expr) => {
                let value = self.eval(*expr);
                println!("{}", value);
                value
            }
        }
    }
}
