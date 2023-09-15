use crate::lexer::{Lexer, TokenKind};

pub type LexerRule = fn(&mut Lexer, char) -> Option<TokenKind>;

pub static LEXER_RULES: &[LexerRule] = &[rule_string_ident];

fn rule_string_ident(lexer: &mut Lexer, char: char) -> Option<TokenKind> {
    None
}
