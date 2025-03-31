use super::parser::{lexer::Token, ast::{Parser, SyntaxNode, AstError}};
use logos::Logos;

pub struct SqlBuilder {
    code: String,
}

impl SqlBuilder {
    pub fn build(self) -> Result<SyntaxNode, AstError> {
        Ok(Parser::from_tokens(&mut Token::lexer(&self.code))?.parse())
    }
}

impl From<String> for SqlBuilder {
    fn from(code: String) -> Self {
        Self { code  }
    }
}
