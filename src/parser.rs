use crate::lexer::{Lexer, Token};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub enum ASTNode {
    Number(i64),
    StringLiteral(String),
    BinaryOp(Rc<RefCell<ASTNode>>, Token, Rc<RefCell<ASTNode>>),
    Identifier(String),
    Assign(String, Rc<RefCell<ASTNode>>),
    Print(Rc<RefCell<ASTNode>>),
    Program(Vec<Rc<RefCell<ASTNode>>>),
    FunctionCall(String, Vec<Rc<RefCell<ASTNode>>>),
    Boolean(bool),
    Comparison(Rc<RefCell<ASTNode>>, Token, Rc<RefCell<ASTNode>>),
    LogicalOp(Rc<RefCell<ASTNode>>, Token, Rc<RefCell<ASTNode>>),
    Not(Rc<RefCell<ASTNode>>),
    Array(Vec<Rc<RefCell<ASTNode>>>),
    IndexAccess(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    If(Rc<RefCell<ASTNode>>, Vec<Rc<RefCell<ASTNode>>>, Vec<(Rc<RefCell<ASTNode>>, Vec<Rc<RefCell<ASTNode>>>)>, Option<Vec<Rc<RefCell<ASTNode>>>>),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, String> {
        let current_token = lexer.next_token()?;
        Ok(Parser { lexer, current_token })
    }

    fn eat(&mut self, token: Token) -> Result<(), String> {
        if self.current_token == token {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(format!("Unexpected token: {:?}, expected: {:?}", self.current_token, token))
        }
    }

    pub fn parse(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.parse_statement()?);
        }
        Ok(Rc::new(RefCell::new(ASTNode::Program(statements))))
    }

    fn parse_statement(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        match &self.current_token {
            Token::If => self.parse_if_statement(),
            Token::Print => {
                self.eat(Token::Print)?;
                let expr = self.parse_expression()?;
                self.eat(Token::Semicolon)?;
                Ok(Rc::new(RefCell::new(ASTNode::Print(expr))))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                if self.current_token == Token::Assign {
                    self.eat(Token::Assign)?;
                    let expr = self.parse_expression()?;
                    self.eat(Token::Semicolon)?;
                    Ok(Rc::new(RefCell::new(ASTNode::Assign(name, expr))))
                } else {
                    // If it's not an assignment, treat it as an expression
                    let expr = self.parse_expression()?;
                    self.eat(Token::Semicolon)?;
                    Ok(expr)
                }
            }
            _ => {
                // For any other token, treat it as an expression
                let expr = self.parse_expression()?;
                self.eat(Token::Semicolon)?;
                Ok(expr)
            }
        }
    }

    fn parse_if_statement(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        self.eat(Token::If)?;
        let condition = self.parse_expression()?;
        self.eat(Token::LBrace)?;
        let if_block = self.parse_block()?;
        let mut else_if_blocks = Vec::new();
        let mut else_block = None;

        while self.current_token == Token::ElseIf {
            self.eat(Token::ElseIf)?;
            let else_if_condition = self.parse_expression()?;
            self.eat(Token::LBrace)?;
            let else_if_block = self.parse_block()?;
            else_if_blocks.push((else_if_condition, else_if_block));
        }

        if self.current_token == Token::Else {
            self.eat(Token::Else)?;
            self.eat(Token::LBrace)?;
            else_block = Some(self.parse_block()?);
        }

        Ok(Rc::new(RefCell::new(ASTNode::If(condition, if_block, else_if_blocks, else_block))))
    }

    fn parse_block(&mut self) -> Result<Vec<Rc<RefCell<ASTNode>>>, String> {
        let mut statements = Vec::new();
        while self.current_token != Token::RBrace {
            statements.push(self.parse_statement()?);
        }
        self.eat(Token::RBrace)?;
        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_logical_and()?;

        while self.current_token == Token::Or {
            let op = self.current_token.clone();
            self.eat(Token::Or)?;
            let right = self.parse_logical_and()?;
            node = Rc::new(RefCell::new(ASTNode::LogicalOp(node, op, right)));
        }

        Ok(node)
    }

    fn parse_logical_and(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_equality()?;

        while self.current_token == Token::And {
            let op = self.current_token.clone();
            self.eat(Token::And)?;
            let right = self.parse_equality()?;
            node = Rc::new(RefCell::new(ASTNode::LogicalOp(node, op, right)));
        }

        Ok(node)
    }

    fn parse_equality(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_comparison()?;

        while self.current_token == Token::Eq || self.current_token == Token::NotEq {
            let op = self.current_token.clone();
            self.eat(self.current_token.clone())?;
            let right = self.parse_comparison()?;
            node = Rc::new(RefCell::new(ASTNode::Comparison(node, op, right)));
        }

        Ok(node)
    }

    fn parse_comparison(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_term()?;

        while matches!(self.current_token, Token::Lt | Token::Gt | Token::LtEq | Token::GtEq) {
            let op = self.current_token.clone();
            self.eat(self.current_token.clone())?;
            let right = self.parse_term()?;
            node = Rc::new(RefCell::new(ASTNode::Comparison(node, op, right)));
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_factor()?;

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.eat(self.current_token.clone())?;
            let right = self.parse_factor()?;
            node = Rc::new(RefCell::new(ASTNode::BinaryOp(node, op, right)));
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_unary()?;

        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let op = self.current_token.clone();
            self.eat(self.current_token.clone())?;
            let right = self.parse_unary()?;
            node = Rc::new(RefCell::new(ASTNode::BinaryOp(node, op, right)));
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        if self.current_token == Token::Not {
            self.eat(Token::Not)?;
            let expr = self.parse_unary()?;
            Ok(Rc::new(RefCell::new(ASTNode::Not(expr))))
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_primary()?;

        loop {
            match &self.current_token {
                Token::LBracket => {
                    self.eat(Token::LBracket)?;
                    let index = self.parse_expression()?;
                    self.eat(Token::RBracket)?;
                    node = Rc::new(RefCell::new(ASTNode::IndexAccess(node, index)));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        match &self.current_token {
            Token::Number(n) => {
                let value = *n;
                self.eat(Token::Number(value))?;
                Ok(Rc::new(RefCell::new(ASTNode::Number(value))))
            }
            Token::StringLiteral(s) => {
                let value = s.clone();
                self.eat(Token::StringLiteral(value.clone()))?;
                Ok(Rc::new(RefCell::new(ASTNode::StringLiteral(value))))
            }
            Token::True => {
                self.eat(Token::True)?;
                Ok(Rc::new(RefCell::new(ASTNode::Boolean(true))))
            }
            Token::False => {
                self.eat(Token::False)?;
                Ok(Rc::new(RefCell::new(ASTNode::Boolean(false))))
            }
            Token::Identifier(name) => {
                let value = name.clone();
                self.eat(Token::Identifier(value.clone()))?;
                Ok(Rc::new(RefCell::new(ASTNode::Identifier(value))))
            }
            Token::LParen => {
                self.eat(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.eat(Token::RParen)?;
                Ok(expr)
            }
            Token::Join | Token::Split | Token::Count => {
                let func_name = match &self.current_token {
                    Token::Join => "join",
                    Token::Split => "split",
                    Token::Count => "count",
                    _ => unreachable!(),
                };
                self.eat(self.current_token.clone())?;
                self.eat(Token::LParen)?;
                let mut args = Vec::new();
                if self.current_token != Token::RParen {
                    args.push(self.parse_expression()?);
                    while self.current_token == Token::Comma {
                        self.eat(Token::Comma)?;
                        args.push(self.parse_expression()?);
                    }
                }
                self.eat(Token::RParen)?;
                Ok(Rc::new(RefCell::new(ASTNode::FunctionCall(func_name.to_string(), args))))
            }
            Token::LBracket => {
                self.eat(Token::LBracket)?;
                let mut elements = Vec::new();
                if self.current_token != Token::RBracket {
                    elements.push(self.parse_expression()?);
                    while self.current_token == Token::Comma {
                        self.eat(Token::Comma)?;
                        elements.push(self.parse_expression()?);
                    }
                }
                self.eat(Token::RBracket)?;
                Ok(Rc::new(RefCell::new(ASTNode::Array(elements))))
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }
}