use crate::parser::{ASTNode};
use crate::lexer::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Evaluator {
    variables: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, node: Rc<RefCell<ASTNode>>) -> Result<Option<Value>, String> {
        match &*node.borrow() {
            ASTNode::Program(statements) => {
                let mut last_result = None;
                for stmt in statements {
                    last_result = self.eval(Rc::clone(stmt))?;
                }
                Ok(last_result)
            }
            ASTNode::Number(value) => Ok(Some(Value::Number(*value))),
            ASTNode::StringLiteral(value) => Ok(Some(Value::String(value.clone()))),
            ASTNode::Boolean(value) => Ok(Some(Value::Boolean(*value))),
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.eval(Rc::clone(left))?.unwrap();
                let right_val = self.eval(Rc::clone(right))?.unwrap();
                let op_clone = op.clone();
                
                // Clone the values before the match statement
                let left_clone = left_val.clone();
                let right_clone = right_val.clone();
                
                match (left_val, op_clone, right_val) {
                    (Value::Number(l), Token::Plus, Value::Number(r)) => Ok(Some(Value::Number(l + r))),
                    (Value::Number(l), Token::Minus, Value::Number(r)) => Ok(Some(Value::Number(l - r))),
                    (Value::Number(l), Token::Star, Value::Number(r)) => Ok(Some(Value::Number(l * r))),
                    (Value::Number(l), Token::Slash, Value::Number(r)) => Ok(Some(Value::Number(l / r))),
                    _ => Err(format!("Invalid operation: {:?} {:?} {:?}", left_clone, op, right_clone)),
                }
            }
            ASTNode::Identifier(name) => {
                Ok(Some(self.variables.get(name).unwrap_or_else(|| panic!("Undefined variable: {}", name)).clone()))
            }
            ASTNode::Assign(name, expr) => {
                let value = self.eval(Rc::clone(expr))?.unwrap();
                self.variables.insert(name.clone(), value.clone());
                Ok(Some(value))
            }
            ASTNode::Print(expr) => {
                if let Some(value) = self.eval(Rc::clone(expr))? {
                    match value {
                        Value::Number(n) => println!("{}", n),
                        Value::String(s) => println!("{}", s),
                        Value::Boolean(b) => println!("{}", b),
                    }
                }
                Ok(None)
            }
            ASTNode::FunctionCall(name, args) => {
                match name.as_str() {
                    "join" => self.join_function(args),
                    "split" => self.split_function(args),
                    "count" => self.count_function(args),
                    _ => Err(format!("Unknown function: {}", name)),
                }
            }
            ASTNode::Comparison(left, op, right) => {
                let left_val = self.eval(Rc::clone(left))?.unwrap();
                let right_val = self.eval(Rc::clone(right))?.unwrap();
                let op_clone = op.clone();
                
                // Clone the values before the match statement
                let left_clone = left_val.clone();
                let right_clone = right_val.clone();
                
                let result = match (left_val, &op_clone, right_val) {
                    (Value::Number(l), Token::Eq, Value::Number(r)) => l == r,
                    (Value::Number(l), Token::NotEq, Value::Number(r)) => l != r,
                    (Value::Number(l), Token::Lt, Value::Number(r)) => l < r,
                    (Value::Number(l), Token::Gt, Value::Number(r)) => l > r,
                    (Value::Number(l), Token::LtEq, Value::Number(r)) => l <= r,
                    (Value::Number(l), Token::GtEq, Value::Number(r)) => l >= r,
                    (Value::String(l), Token::Eq, Value::String(r)) => l == r,
                    (Value::String(l), Token::NotEq, Value::String(r)) => l != r,
                    (Value::Boolean(l), Token::Eq, Value::Boolean(r)) => l == r,
                    (Value::Boolean(l), Token::NotEq, Value::Boolean(r)) => l != r,
                    _ => return Err(format!("Invalid comparison: {:?} {:?} {:?}", left_clone, op_clone, right_clone)),
                };
                Ok(Some(Value::Boolean(result)))
            }
            ASTNode::LogicalOp(left, op, right) => {
                let left_val = self.eval(Rc::clone(left))?.unwrap();
                let op_clone = op.clone();
                
                // Clone the value before the match statement
                let left_clone = left_val.clone();
                
                match (left_val, &op_clone) {
                    (Value::Boolean(true), Token::Or) => Ok(Some(Value::Boolean(true))),
                    (Value::Boolean(false), Token::Or) => self.eval(Rc::clone(right)),
                    (Value::Boolean(true), Token::And) => self.eval(Rc::clone(right)),
                    (Value::Boolean(false), Token::And) => Ok(Some(Value::Boolean(false))),
                    _ => Err(format!("Invalid logical operation: {:?} {:?}", left_clone, op_clone)),
                }
            }
            ASTNode::Not(expr) => {
                let val = self.eval(Rc::clone(expr))?.unwrap();
                match val {
                    Value::Boolean(b) => Ok(Some(Value::Boolean(!b))),
                    _ => Err(format!("Cannot apply 'not' to non-boolean value: {:?}", val)),
                }
            }
        }
    }

    fn join_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 2 {
            return Err("join function requires 2 arguments".to_string());
        }
        let separator = match self.eval(Rc::clone(&args[0]))?.unwrap() {
            Value::String(s) => s,
            _ => return Err("First argument of join must be a string".to_string()),
        };
        let elements = match self.eval(Rc::clone(&args[1]))?.unwrap() {
            Value::String(s) => s.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>(),
            _ => return Err("Second argument of join must be a string".to_string()),
        };
        Ok(Some(Value::String(elements.join(&separator))))
    }

    fn split_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 2 {
            return Err("split function requires 2 arguments".to_string());
        }
        let string = match self.eval(Rc::clone(&args[0]))?.unwrap() {
            Value::String(s) => s,
            _ => return Err("First argument of split must be a string".to_string()),
        };
        let separator = match self.eval(Rc::clone(&args[1]))?.unwrap() {
            Value::String(s) => s,
            _ => return Err("Second argument of split must be a string".to_string()),
        };
        Ok(Some(Value::String(string.split(&separator).collect::<Vec<&str>>().join(","))))
    }

    fn count_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 2 {
            return Err("count function requires 2 arguments".to_string());
        }
        let string = match self.eval(Rc::clone(&args[0]))?.unwrap() {
            Value::String(s) => s,
            _ => return Err("First argument of count must be a string".to_string()),
        };
        let substring = match self.eval(Rc::clone(&args[1]))?.unwrap() {
            Value::String(s) => s,
            _ => return Err("Second argument of count must be a string".to_string()),
        };
        Ok(Some(Value::Number(string.matches(&substring).count() as i64)))
    }
}