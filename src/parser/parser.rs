use crate::lexer::lexer::Token; 

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),          // The entire program (a list of statements)
    StmtList(Vec<ASTNode>),         // A list of statements
    LetStmt(String, Box<ASTNode>),  // let statement: variable name and expression
    IfStmt(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>), // if condition, then block, else block (optional)
    WhileStmt(Box<ASTNode>, Box<ASTNode>),  // while loop: condition and body
    ReturnStmt(Box<ASTNode>),       // return statement: expression
    Block(Vec<ASTNode>),            // Block of statements
    ExprStmt(Box<ASTNode>),         // Expression statement
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),  // Binary operation: left operand, operator, right operand
    Identifier(String),             // Identifier (variable name)
    Integer(i64),                   // Integer literal
    StringLiteral(String),          // String literal
}

pub struct Parser {
    tokens: Vec<Token>,  // List of tokens from the lexer
    current_token: usize,  // Current token position
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_token: 0,
        }
    }

    fn advance(&mut self) {
        if self.current_token < self.tokens.len() {
            self.current_token += 1;
        }
    }

    fn current(&self) -> &Token {
        if self.current_token < self.tokens.len() {
            &self.tokens[self.current_token]
        } else {
            &Token::EOF // Return EOF if we've gone past the end
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if *self.current() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current()))
        }
    }

    pub fn parse_program(&mut self) -> Result<ASTNode, String> {
        self.parse_stmt_list()
    }

    fn parse_stmt_list(&mut self) -> Result<ASTNode, String> {
        let mut stmts = Vec::new();
        while self.current() != &Token::EOF && self.current() != &Token::RBrace {
            stmts.push(self.parse_stmt()?);
        }
        Ok(ASTNode::StmtList(stmts))
    }

    fn parse_stmt(&mut self) -> Result<ASTNode, String> {
        match self.current() {
            Token::Let => self.parse_let_stmt(),
            Token::If => self.parse_if_stmt(),
            Token::While => self.parse_while_stmt(),
            Token::Return => self.parse_return_stmt(),
            Token::LBrace => self.parse_block(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::Let)?;
        let identifier = if let Token::Identifier(name) = self.current() {
            name.clone()
        } else {
            return Err("Expected identifier".to_string());
        };
        self.advance();
        self.expect(Token::Equal)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(ASTNode::LetStmt(identifier, Box::new(expr)))
    }

    fn parse_if_stmt(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::If)?;
        self.expect(Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let then_branch = self.parse_stmt()?;
        let else_branch = if self.current() == &Token::Else {
            self.advance();
            Some(self.parse_stmt()?)
        } else {
            None
        };
        Ok(ASTNode::IfStmt(Box::new(condition), Box::new(then_branch), else_branch.map(Box::new)))
    }

    fn parse_while_stmt(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let body = self.parse_stmt()?;
        Ok(ASTNode::WhileStmt(Box::new(condition), Box::new(body)))
    }

    fn parse_return_stmt(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::Return)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(ASTNode::ReturnStmt(Box::new(expr)))
    }

    fn parse_block(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::LBrace)?;
        let mut stmts = Vec::new();
        while self.current() != &Token::RBrace {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(ASTNode::Block(stmts))
    }

    fn parse_expr_stmt(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(ASTNode::ExprStmt(Box::new(expr)))
    }

    fn parse_expr(&mut self) -> Result<ASTNode, String> {
        let mut node = self.parse_term()?;
        while let Token::Plus | Token::Minus | Token::GreaterThan | Token::LessThan | Token::Equal | Token::NotEqual = self.current() {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_term()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
        }
        Ok(node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut node = self.parse_factor()?;
        while let Token::Star | Token::Slash = self.current() {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_factor()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
        }
        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        match self.current() {
            Token::LParen => {
                self.advance();
                let node = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(node)
            }
            Token::Identifier(name) => {
                let node = ASTNode::Identifier(name.clone());
                self.advance();
                Ok(node)
            }
            Token::Integer(value) => {
                let node = ASTNode::Integer(*value);
                self.advance();
                Ok(node)
            }
            Token::StringLiteral(value) => {
                let node = ASTNode::StringLiteral(value.clone());
                self.advance();
                Ok(node)
            }
            _ => Err("Unexpected token in factor".to_string()),
        }
    }
}


