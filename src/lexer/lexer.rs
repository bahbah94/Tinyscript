#[derive(Debug, PartialEq,Clone)]
pub enum Token{
	
	//keywords
	Let,
	If,
	Fn,
	Else,
	While,
	Return,


	// Identifiers
    Identifier(String),

    // Literals
    Integer(i64),
    StringLiteral(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    // End of file
    EOF,
}


//Now we define out input string
pub struct Lexer {
	input: String,
	position: usize,
	current_char: Option<char>,
}


// First impl block for basic utility methods like `advance`
impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.advance(); // Initialize by advancing to the first character
        lexer
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        } else {
            self.current_char = None; // End of input
        }
    }
}

// Second impl block for tokenization methods
impl Lexer {
    pub fn get_next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\n' | '\r' => self.advance(), 

                // Operators
                '+' => {
                    self.advance();
                    return Token::Plus;
                },
                '-' => {
                    self.advance();
                    return Token::Minus;
                },
                '*' => {
                    self.advance();
                    return Token::Star;
                },
                '/' => {
                    self.advance();
                    return Token::Slash;
                },
                '=' => {
                    self.advance();
                    return Token::Equal;
                },

                '>' => {
                    self.advance();
                    return Token::GreaterThan
                },
                '<' => {
                    self.advance();
                    return Token::LessThan
                },

                '!' => {
                    self.advance();
                    return Token::NotEqual
                },

                // Delimiters
                '(' => {
                    self.advance();
                    return Token::LParen;
                },
                ')' => {
                    self.advance();
                    return Token::RParen;
                },
                '{' => {
                    self.advance();
                    return Token::LBrace;
                },
                '}' => {
                    self.advance();
                    return Token::RBrace;
                },
                ',' => {
                    self.advance();
                    return Token::Comma;
                },
                ';' => {
                    self.advance();
                    return Token::Semicolon;
                },

                // String literals
                '"' => {
                    return self.string_literal();
                },

                // Integer literals
                '0'..='9' => {
                    return self.integer_literal();
                },

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    return self.identifier();
                },

                // Handle unexpected characters
                _ => {
                    panic!("Unexpected character: {}", ch);
                }
            }
        }

        Token::EOF
    }

    fn identifier(&mut self) -> Token {
        let mut result = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match result.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "return" => Token::Return,
            _ => Token::Identifier(result),
        }
    }

    fn integer_literal(&mut self) -> Token {
        let mut result = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::Integer(result.parse::<i64>().unwrap())
    }

    fn string_literal(&mut self) -> Token {
        let mut result = String::new();
        self.advance(); // Skip the opening quote

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip the closing quote
                return Token::StringLiteral(result);
            } else {
                result.push(ch);
                self.advance();
            }
        }

        panic!("Unterminated string literal");
    }
}

