use std::{
    collections::VecDeque,
    iter::{Enumerate, Peekable},
    str::Chars,
};

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

use self::TokenKind::*;

pub struct Cursor<'a> {
    source: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Cursor<'a> {
    pub fn new(source_code: &'a str) -> Cursor<'a> {
        Cursor {
            source: source_code.chars().enumerate().peekable(),
        }
    }

    /// [Iterator::take_while] that does not consume non-matching items
    /// by peeking
    pub fn take_while(&mut self, predicate: fn(char) -> bool) -> Vec<char> {
        let mut chars = Vec::new();

        loop {
            let Some(char) = self.source.peek()
            else {
                return chars;
            };

            if predicate(char.1) {
                chars.push(self.source.next().unwrap().1);
            } else {
                return chars;
            }
        }
    }

    pub fn skip_whitespace(&mut self) -> usize {
        let mut indent_level = 0;

        while let Some((_, char)) = self.source.peek() {
            match char {
                ' ' => {
                    self.source.next();
                    indent_level += 1;
                }
                _ => break,
            }
        }

        return indent_level;
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

    token_queue: VecDeque<Token>,

    /// Indentation is meant to work exactly like [Python indentation](https://docs.python.org/3/reference/lexical_analysis.html#indentation).
    indentation_stack: Vec<usize>,

    is_line_start: bool,
    line_start_idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Lexer<'a> {
        Lexer {
            cursor: Cursor::new(source_code),
            custom_rules: rules::LEXER_RULES,
            src: source_code,
            token_queue: VecDeque::new(),
            indentation_stack: vec![0],
            is_line_start: true,
            line_start_idx: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.token_queue.pop_front() {
            return Some(token);
        }

        let indent_level = self.cursor.skip_whitespace();

        if self.is_line_start {
            if indent_level > *self.indentation_stack.last()? {
                self.is_line_start = false;
                self.indentation_stack.push(indent_level);

                return Some(Token {
                    kind: Indent,
                    span: Span::new(self.line_start_idx, self.line_start_idx),
                });
            } else if indent_level < *self.indentation_stack.last()? {
                let remaining_stack: Vec<usize> = self
                    .indentation_stack
                    .iter()
                    .filter_map(|stacked_indent| {
                        if stacked_indent <= &indent_level {
                            Some(*stacked_indent)
                        } else {
                            None
                        }
                    })
                    .collect();

                let popped_count = self.indentation_stack.len() - remaining_stack.len();

                for _ in 0..popped_count {
                    self.token_queue.push_back(Token {
                        kind: Dedent,
                        span: Span::new(self.line_start_idx, self.line_start_idx),
                    })
                }

                self.indentation_stack = remaining_stack;

                if let Some(token) = self.token_queue.pop_front() {
                    return Some(token);
                }
            }
        }

        self.is_line_start = false;
        let (end_idx, char) = self.cursor.source.next()?;
        let mut end_idx = end_idx;
        let start_idx = end_idx;

        let token_kind = match char {
            char if is_ident_start(char) => {
                let mut chars: Vec<char> = self.cursor.take_while(is_ident_body);
                end_idx = end_idx + chars.len();

                chars.insert(0, char);

                let ident_str = String::from_iter(&chars);
                Identifier(ident_str)
            }
            char @ '0'..='9' => {
                let mut chars: Vec<char> = self.cursor.take_while(is_digit);
                end_idx = end_idx + chars.len();

                chars.insert(0, char);

                let int_str = String::from_iter(&chars);
                IntegerLiteral(int_str)
            }
            '+' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    PlusEquals
                }
                _ => Plus,
            },
            '-' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    MinusEquals
                }
                Some((_, '>')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    RArrow
                }
                _ => Minus,
            },
            '*' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    StarEquals
                }
                Some((_, '*')) => {
                    (end_idx, _) = self.cursor.source.next()?;

                    match self.cursor.source.peek() {
                        Some((_, '=')) => {
                            (end_idx, _) = self.cursor.source.next()?;
                            StarStarEquals
                        }
                        _ => StarStar,
                    }
                }
                _ => Star,
            },
            '/' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    SlashEquals
                }
                _ => Slash,
            },
            '(' => LParen,
            ')' => RParen,
            '[' => LBracket,
            ']' => RBracket,
            '{' => LBrace,
            '}' => RBrace,
            ':' => Colon,
            ';' => Semi,
            '.' => Dot,
            ',' => Comma,
            '%' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    PercentEquals
                }
                _ => Percent,
            },
            '^' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    CaretEquals
                }
                _ => Caret,
            },
            '&' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    AmperEquals
                }
                _ => Amper,
            },
            '|' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    PipeEquals
                }
                _ => Pipe,
            },
            '~' => Tilde,
            '=' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    EqualsEquals
                }
                _ => Equals,
            },
            '<' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    LessEquals
                }
                Some((_, '<')) => {
                    (end_idx, _) = self.cursor.source.next()?;

                    match self.cursor.source.peek() {
                        Some((_, '=')) => {
                            (end_idx, _) = self.cursor.source.next()?;
                            LShiftEquals
                        }
                        _ => LShift,
                    }
                }
                _ => Less,
            },
            '>' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    GreaterEquals
                }
                Some((_, '>')) => {
                    (end_idx, _) = self.cursor.source.next()?;

                    match self.cursor.source.peek() {
                        Some((_, '=')) => {
                            (end_idx, _) = self.cursor.source.next()?;
                            RShiftEquals
                        }
                        _ => RShift,
                    }
                }
                _ => Greater,
            },
            '!' => match self.cursor.source.peek() {
                Some((_, '=')) => {
                    (end_idx, _) = self.cursor.source.next()?;
                    NotEquals
                }
                _ => Not,
            },
            '@' => At,
            '\n' => {
                self.is_line_start = true;
                self.line_start_idx = start_idx + 1;
                NewLine
            }
            _ => Unknown,
        };

        let span = Span::new(start_idx, end_idx + 1);

        if self.cursor.source.peek().is_none() {
            self.token_queue.push_back(Token {
                kind: NewLine,
                span: Span::new(end_idx + 1, end_idx + 2),
            });

            for _ in 0..self.indentation_stack.len() - 1 {
                self.token_queue.push_back(Token {
                    kind: Dedent,
                    span: Span::new(end_idx + 2, end_idx + 2),
                })
            }
        }

        return Some(Token {
            kind: token_kind,
            span,
        });
    }
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn is_ident_body(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}
