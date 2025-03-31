use mantel::parser;
use logos::Logos;
use rowan::NodeOrToken;

fn print(indent: usize, element: mantel::parser::ast::SyntaxElement) {
    let kind: mantel::parser::ast::SyntaxKind = element.kind();
    print!("{:indent$}", "", indent = indent);
    match element {
        NodeOrToken::Node(node) => {
            println!("- {:?}", kind);
            for child in node.children_with_tokens() {
                print(indent + 2, child);
            }
        }

        NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
    }
}

fn main() {
    let mut lex = parser::lexer::Token::lexer("SeLect * FROM table");
    let parser = parser::ast::Parser::from_tokens(&mut lex).unwrap().parse();
    print(0, parser.into());
}
