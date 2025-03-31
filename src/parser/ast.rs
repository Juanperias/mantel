// Select
//    All - *
//    From
//      Identifier - Table
use super::lexer::Token;
use logos::Lexer;
use rowan::{GreenNodeBuilder, NodeOrToken};
use std::iter::Peekable;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Invalid Token {0}")]
    InvalidToken(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    ROOT,
    WHITESPACE,
    SELECT,
    FROM,
    IDENTIFIER,
    TEXT,
    ALL,
}

use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

type SyntaxNode = rowan::SyntaxNode<Lang>;
type SyntaxToken = rowan::SyntaxToken<Lang>;
type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

pub struct Parser<I: Iterator<Item = (SyntaxKind, String)>> {
    builder: GreenNodeBuilder<'static>,
    iter: Peekable<I>,
}

impl<I: Iterator<Item = (SyntaxKind, String)>> Parser<I> {
    fn peek(&mut self) -> Option<SyntaxKind> {
        while self
            .iter
            .peek()
            .map(|&(t, _)| t == WHITESPACE)
            .unwrap_or(false)
        {
            self.bump();
        }
        self.iter.peek().map(|&(t, _)| t)
    }
    fn bump(&mut self) {
        if let Some((token, string)) = self.iter.next() {
            self.builder.token(token.into(), string.as_str());
        }
    }
    fn handle_val(&mut self) {
        match self.peek () {
            _ => {},
        }
    }
    fn parse(mut self) -> SyntaxNode {
        self.builder.start_node(ROOT.into());

        while let Some(_) = self.peek() {
            self.handle_val()
        }

        self.builder.finish_node();

        SyntaxNode::new_root(self.builder.finish())
    }
    fn from_tokens(lex: &mut Lexer<'_, Token>) -> Result<Parser<std::vec::IntoIter<(SyntaxKind, String)>>, AstError> {
        let mut nodes = Vec::new();

        while let Some(token) = lex.next() {
            match token {
                Ok(t) => {
                    nodes.push(t.to_syntax());
                }
                Err(_) => return Err(AstError::InvalidToken(lex.slice().to_string())),
            }
        }

        Ok(Parser {
            builder: GreenNodeBuilder::new(),
            iter: nodes.into_iter().peekable(),
        })
    }
}
