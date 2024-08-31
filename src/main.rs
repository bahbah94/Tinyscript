// mod lexer;  // Import the lexer module
// mod parser; // Import the parser module

// use lexer::lexer::{Lexer, Token}; // Import Lexer and Token from the lexer module
// use parser::parser::{Parser}; // Import Parser and ASTNode from the parser module

// fn main() {
//     // Example input string
//     let input = String::from(r#"
//         let x = 42;
//         if (x > 10) {
//             return "Hello, World";
//         }
//     "#);

//     // Initialize the lexer with the input string
//     let mut lexer = Lexer::new(input);
//     let mut tokens = Vec::new();

//     // Collect tokens from the lexer
//     loop {
//         let token = lexer.get_next_token();
//         if token == Token::EOF {
//             break;
//         }
//         tokens.push(token);
//     }

//     // Initialize the parser with the tokens
//     let mut parser = Parser::new(tokens);
//     match parser.parse_program() {
//         Ok(ast) => println!("AST: {:?}", ast),  // Print the resulting AST
//         Err(e) => println!("Error: {}", e),    // Print error if parsing fails
//     }
// }

mod lexer;
mod parser;
mod typecheck;

use lexer::lexer::{Lexer, Token};
use parser::parser::{Parser};
use typecheck::typecheck::{SemanticAnalyzer};

fn main() {
    // Example TinyScript program
    let input = String::from(r#"
        let x = 42;
        if (x > 10) {
            let y = x + 5;
            print(y);
        }
    "#);

    // Lexing: Convert input source code to tokens
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

//     // Collect tokens from the lexer
     loop {
        let token = lexer.get_next_token();
         if token == Token::EOF {
                      break;
         }
         tokens.push(token);
    } // Collect all tokens into a vector

    // Parsing: Convert tokens into an Abstract Syntax Tree (AST)
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().expect("Parsing failed");

    // Semantic Analysis: Type-checking and scope resolution
    let mut analyzer = SemanticAnalyzer::new();
    if let Err(e) = analyzer.check_types(&ast) {
        println!("Semantic error: {}", e);
    } else {
        println!("Semantic analysis passed");
    }

    // If everything passes, you can proceed with code generation or execution
    println!("{:#?}", ast);
}


