mod lexer;

use lexer::lexer::{Lexer,Token};

fn main() {
    

    let input = String :: from("let x = 10 + 20;");

    let mut lexer = Lexer:: new(input);


    loop {
        let token = lexer.get_next_token();
        println!("{:?}", token); // Print the token for debugging
        
        if token == Token::EOF {
            break; // Stop when we reach the end of the file/input
        }
    }
}

