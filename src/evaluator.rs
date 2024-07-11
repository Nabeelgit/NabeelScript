use crate::parser::{ASTNode};
use crate::lexer::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Evaluator {
    variables: HashMap<String, i64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, node: Rc<RefCell<ASTNode>>) -> Option<i64> {
        match &*node.borrow() {
            ASTNode::Program(statements) => {
                let mut last_result = None;
                for stmt in statements {
                    last_result = self.eval(Rc::clone(stmt));
                }
                last_result
            }
            ASTNode::Number(value) => Some(*value),
            ASTNode::StringLiteral(value) => {
                println!("{}", value);
                None
            }
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.eval(Rc::clone(left)).unwrap();
                let right_val = self.eval(Rc::clone(right)).unwrap();
                let result = match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    _ => panic!("Unknown operator: {:?}", op),
                };
                Some(result)
            }
            ASTNode::Identifier(name) => {
                Some(*self.variables.get(name).unwrap_or_else(|| panic!("Undefined variable: {}", name)))
            }
            ASTNode::Assign(name, expr) => {
                let value = self.eval(Rc::clone(expr)).unwrap();
                self.variables.insert(name.clone(), value);
                Some(value)
            }
            ASTNode::Print(expr) => {
                if let Some(value) = self.eval(Rc::clone(expr)) {
                    println!("{}", value);
                }
                None
            }
        }
    }
}