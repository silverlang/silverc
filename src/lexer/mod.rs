use std::{collections::HashMap, rc::Rc, str::Chars};

use crate::span::Span;

use self::rules::LexerRule;
mod rules;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub enum TokenKind {
    IntegerLiteral(Rc<str>),
    StringLiteral(Rc<str>),
    Identifier(Rc<str>),
}

pub struct Cursor<'a> {
    source: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(source_code: &'a str) -> Cursor<'a> {
        Cursor {
            source: source_code.chars(),
        }
    }
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,

    /// Source code to tokenize.
    src: &'a str,

    /// A vector of custom lexer rule functions.
    ///
    /// The Lexer will execute each function, and if a function returns [Some] it will create a [Token] with
    /// the [TokenKind] returned.
    custom_rules: &'a [LexerRule],
}

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Lexer<'a> {
        Lexer {
            cursor: Cursor::new(source_code),
            custom_rules: rules::LEXER_RULES,
            src: source_code,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {}
}
