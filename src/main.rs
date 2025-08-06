mod lexer;

use lexer::lexer::Lexer;
use lexer::token::Token;

fn main() {
    let input = "
        + - * / % & | ^ ! = 
        ( ) { } [ ] , ;
    ";

    let mut lexer = Lexer::new(input);

    println!("--- Tokens ---");
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}
