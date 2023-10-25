use std::str::Chars;

use crate::span::Span;

use self::rules::LexerRule;
mod rules;

#[cfg(test)]
mod test;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum TokenKind {
    Identifier(String),
    IntegerLiteral(String),
    StringLiteral(String),

    /// \n
    NewLine,

    NL,

    Indent,

    Dedent,

    /// "("
    LParen,

    /// ")"
    RParen,

    /// "["
    LBracket,

    /// "]"
    RBracket,

    /// "{"
    LBrace,

    /// "}"
    RBrace,

    /// ":"
    Colon,

    /// ";"
    Semi,

    /// "."
    Dot,

    /// ","
    Comma,

    /// "+"
    Plus,

    /// "-"
    Minus,

    /// "*"
    Star,

    /// "/"
    Slash,

    /// "%"
    Percent,

    /// "^"
    Caret,

    /// "&"
    Amper,

    /// "|"
    Pipe,

    /// "~"
    Tilde,

    /// "="
    Equals,

    /// "<"
    Less,

    /// ">"
    Greater,

    /// "!"
    Not,

    /// "@"
    At,

    /// "->"
    RArrow,

    /// "=="
    EqualsEquals,

    /// "!="
    NotEquals,

    /// "<="
    LessEquals,

    /// ">="
    GreaterEquals,

    /// "<<"
    LShift,

    /// ">>"
    RShift,

    /// "**"
    StarStar,

    /// "+="
    PlusEquals,

    /// "-="
    MinusEquals,

    /// "*="
    StarEquals,

    /// "/="
    SlashEquals,

    /// "%="
    PercentEquals,

    /// "&="
    AmperEquals,

    /// "|="
    PipeEquals,

    /// "^="
    CaretEquals,

    /// "<<="
    LShiftEquals,

    /// ">>="
    RShiftEquals,

    /// "**="
    StarStarEquals,

    Unknown,
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
