use mantel::parser;
use logos::Logos;

fn main() {
    let mut lex = parser::lexer::Token::lexer("SeLect * FROM table");

    while let Some(token) = lex.next() {
        println!("{:?}", token);
    }
}
