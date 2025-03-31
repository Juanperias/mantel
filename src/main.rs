use mantel::SqlBuilder;
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

fn main() -> Result<(), mantel::parser::ast::AstError> {
    let ast = SqlBuilder::from("SELECT * FROM 23".to_string())
        .build()?;
    print(0, ast.into());

    Ok(())
}
