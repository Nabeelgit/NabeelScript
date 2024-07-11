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
            Token::Print => {
                self.eat(Token::Print)?;
                let expr = self.parse_expression()?;
                self.eat(Token::Semicolon)?;
                Ok(Rc::new(RefCell::new(ASTNode::Print(expr))))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                self.eat(Token::Assign)?;
                let expr = self.parse_expression()?;
                self.eat(Token::Semicolon)?;
                Ok(Rc::new(RefCell::new(ASTNode::Assign(name, expr))))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.eat(Token::Semicolon)?;
                Ok(expr)
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_term()?;

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = Rc::new(RefCell::new(ASTNode::BinaryOp(node, token, self.parse_term()?)));
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        let mut node = self.parse_factor()?;

        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = Rc::new(RefCell::new(ASTNode::BinaryOp(node, token, self.parse_factor()?)));
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<Rc<RefCell<ASTNode>>, String> {
        match &self.current_token {
            Token::Number(value) => {
                let value = *value;
                self.eat(Token::Number(value))?;
                Ok(Rc::new(RefCell::new(ASTNode::Number(value))))
            }
            Token::StringLiteral(value) => {
                let value = value.clone();
                self.eat(Token::StringLiteral(value.clone()))?;
                Ok(Rc::new(RefCell::new(ASTNode::StringLiteral(value))))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                Ok(Rc::new(RefCell::new(ASTNode::Identifier(name))))
            }
            Token::LParen => {
                self.eat(Token::LParen)?;
                let node = self.parse_expression()?;
                self.eat(Token::RParen)?;
                Ok(node)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }
}