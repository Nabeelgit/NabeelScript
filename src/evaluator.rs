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
    Array(Vec<Value>),
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
                        Value::Array(arr) => println!("{:?}", arr),
                    }
                }
                Ok(None)
            }
            ASTNode::FunctionCall(name, args) => {
                match name.as_str() {
                    "join" => self.join_function(args),
                    "split" => self.split_function(args),
                    "count" => self.count_function(args),
                    "length" => self.length_function(args),
                    "uppercase" => self.uppercase_function(args),
                    "lowercase" => self.lowercase_function(args),
                    "trim" => self.trim_function(args),
                    "replace" => self.replace_function(args),
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
            ASTNode::Array(elements) => {
                let mut array_values = Vec::new();
                for element in elements {
                    if let Some(value) = self.eval(Rc::clone(element))? {
                        array_values.push(value);
                    }
                }
                Ok(Some(Value::Array(array_values)))
            }
            ASTNode::IndexAccess(array, index) => {
                let array_value = self.eval(Rc::clone(array))?.unwrap();
                let index_value = self.eval(Rc::clone(index))?.unwrap();
                match (array_value, index_value) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        if idx < 0 || idx >= arr.len() as i64 {
                            Err(format!("Index out of bounds: {}", idx))
                        } else {
                            Ok(Some(arr[idx as usize].clone()))
                        }
                    }
                    _ => Err(format!("Invalid index access")),
                }
            }
            ASTNode::If(condition, if_block, else_if_blocks, else_block) => {
                if self.eval_boolean_expression(Rc::clone(condition))? {
                    self.eval_block(if_block)
                } else {
                    for (else_if_condition, else_if_block) in else_if_blocks {
                        if self.eval_boolean_expression(Rc::clone(else_if_condition))? {
                            return self.eval_block(else_if_block);
                        }
                    }
                    if let Some(else_block) = else_block {
                        self.eval_block(else_block)
                    } else {
                        Ok(None)
                    }
                }
            }
            ASTNode::While(condition, block) => {
                while self.eval_boolean_expression(Rc::clone(condition))? {
                    self.eval_block(block)?;
                }
                Ok(None)
            }
            ASTNode::For(init, condition, update, block) => {
                self.eval(Rc::clone(init))?;
                while self.eval_boolean_expression(Rc::clone(condition))? {
                    self.eval_block(block)?;
                    self.eval(Rc::clone(update))?;
                }
                Ok(None)
            }
        }
    }

    fn eval_boolean_expression(&mut self, node: Rc<RefCell<ASTNode>>) -> Result<bool, String> {
        match self.eval(node)? {
            Some(Value::Boolean(b)) => Ok(b),
            _ => Err("Expected a boolean expression".to_string()),
        }
    }

    fn eval_block(&mut self, block: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        let mut result = None;
        for statement in block {
            result = self.eval(Rc::clone(statement))?;
        }
        Ok(result)
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
            Value::Array(arr) => arr,
            _ => return Err("Second argument of join must be an array".to_string()),
        };
        
        let joined_string = elements.iter().map(|value| match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(_) => "[array]".to_string(), // You might want to handle nested arrays differently
        }).collect::<Vec<String>>().join(&separator);
        
        Ok(Some(Value::String(joined_string)))
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
        let result: Vec<Value> = string.split(&separator)
            .map(|s| Value::String(s.to_string()))
            .collect();
        Ok(Some(Value::Array(result)))
    }

    fn count_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 2 {
            return Err("count function requires 2 arguments".to_string());
        }
        let first_arg = self.eval(Rc::clone(&args[0]))?.unwrap();
        let second_arg = self.eval(Rc::clone(&args[1]))?.unwrap();

        match (first_arg, second_arg) {
            (Value::String(s), Value::String(substr)) => {
                Ok(Some(Value::Number(s.matches(&substr).count() as i64)))
            }
            (Value::Array(arr), Value::String(substr)) => {
                let count = arr.iter().filter(|&v| {
                    if let Value::String(s) = v {
                        s == &substr
                    } else {
                        false
                    }
                }).count();
                Ok(Some(Value::Number(count as i64)))
            }
            _ => Err("count function arguments must be (string, string) or (array, string)".to_string()),
        }
    }

    fn length_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 1 {
            return Err("length function requires 1 argument".to_string());
        }
        let arg = self.eval(Rc::clone(&args[0]))?.unwrap();
        match arg {
            Value::String(s) => Ok(Some(Value::Number(s.len() as i64))),
            _ => Err("length function argument must be a string".to_string()),
        }
    }

    fn uppercase_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 1 {
            return Err("uppercase function requires 1 argument".to_string());
        }
        let arg = self.eval(Rc::clone(&args[0]))?.unwrap();
        match arg {
            Value::String(s) => Ok(Some(Value::String(s.to_uppercase()))),
            _ => Err("uppercase function argument must be a string".to_string()),
        }
    }

    fn lowercase_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 1 {
            return Err("lowercase function requires 1 argument".to_string());
        }
        let arg = self.eval(Rc::clone(&args[0]))?.unwrap();
        match arg {
            Value::String(s) => Ok(Some(Value::String(s.to_lowercase()))),
            _ => Err("lowercase function argument must be a string".to_string()),
        }
    }

    fn trim_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 1 {
            return Err("trim function requires 1 argument".to_string());
        }
        let arg = self.eval(Rc::clone(&args[0]))?.unwrap();
        match arg {
            Value::String(s) => Ok(Some(Value::String(s.trim().to_string()))),
            _ => Err("trim function argument must be a string".to_string()),
        }
    }

    fn replace_function(&mut self, args: &[Rc<RefCell<ASTNode>>]) -> Result<Option<Value>, String> {
        if args.len() != 3 {
            return Err("replace function requires 3 arguments".to_string());
        }
        let string = self.eval(Rc::clone(&args[0]))?.unwrap();
        let pattern = self.eval(Rc::clone(&args[1]))?.unwrap();
        let replacement = self.eval(Rc::clone(&args[2]))?.unwrap();
        match (string, pattern, replacement) {
            (Value::String(s), Value::String(p), Value::String(r)) => {
                Ok(Some(Value::String(s.replace(&p, &r))))
            }
            _ => Err("replace function arguments must be strings".to_string()),
        }
    }
}