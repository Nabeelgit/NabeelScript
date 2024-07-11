use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum ASTNode {
    Number(i64),
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    Identifier(String),
    Assign(String, Box<ASTNode>),
    Print(Box<ASTNode>),
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

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        self.parse_statements()
    }

    fn parse_statements(&mut self) -> Result<ASTNode, String> {
        let node = self.parse_statement()?;
        while self.current_token != Token::EOF {
            let next_node = self.parse_statement()?;
            // Combine nodes if needed, for now we assume a single statement
            return Ok(next_node);
        }
        Ok(node)
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match &self.current_token {
            Token::Print => {
                self.eat(Token::Print)?;
                let expr = self.parse_expression()?;
                self.eat(Token::Semicolon)?;
                Ok(ASTNode::Print(Box::new(expr)))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                self.eat(Token::Assign)?;
                let expr = self.parse_expression()?;
                self.eat(Token::Semicolon)?;
                Ok(ASTNode::Assign(name, Box::new(expr)))
            }
            _ => self.parse_expression(),
        }
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut node = self.parse_term()?;

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = ASTNode::BinaryOp(Box::new(node), token, Box::new(self.parse_term()?));
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut node = self.parse_factor()?;

        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let token = self.current_token.clone();
            self.eat(token.clone())?;
            node = ASTNode::BinaryOp(Box::new(node), token, Box::new(self.parse_factor()?));
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        match &self.current_token {
            Token::Number(value) => {
                let value = *value;
                self.eat(Token::Number(value))?;
                Ok(ASTNode::Number(value))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                Ok(ASTNode::Identifier(name))
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