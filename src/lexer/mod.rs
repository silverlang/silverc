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
    /// "# comment"
    Comment,

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
    chars: Peekable<Enumerate<Chars<'a>>>,
    idx: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source_code: &'a str) -> Cursor<'a> {
        Cursor {
            chars: source_code.chars().enumerate().peekable(),
            idx: 0,
        }
    }

    /// [Iterator::take_while] that does not consume non-matching items
    /// by peeking
    pub fn take_while(&mut self, predicate: fn(char) -> bool) -> Vec<char> {
        let mut chars = Vec::new();

        loop {
            let Some(char) = self.chars.peek()
            else {
                return chars;
            };

            if predicate(char.1) {
                chars.push(self.bump().unwrap());
            } else {
                return chars;
            }
        }
    }

    /// Advances to the next character.
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        self.idx += 1;

        return Some(c.1);
    }

    pub fn skip_whitespace(&mut self) -> usize {
        let mut indent_level = 0;

        while let Some((_, char)) = self.chars.peek() {
            match char {
                ' ' => {
                    self.bump();
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
            } else if self.indentation_stack.contains(&indent_level) {
                let remaining_stack: Vec<usize> = self
                    .indentation_stack
                    .iter()
                    .take_while(|i| i <= &&indent_level)
                    .map(|i| *i)
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
            } else {
                // TODO: make a proper error for this
                panic!("Inconsistent dedent");
            }
        }

        self.is_line_start = false;
        let start_idx = self.cursor.idx;
        let char = self.cursor.bump()?;

        let token_kind = match char {
            '#' => {
                self.cursor.take_while(|c| c != '\n');
                // TODO: get proper end_idx for this
                Comment
            }
            char if is_ident_start(char) => {
                let mut chars: Vec<char> = self.cursor.take_while(is_ident_body);

                chars.insert(0, char);

                let ident_str = String::from_iter(&chars);
                Identifier(ident_str)
            }
            char @ '0'..='9' => {
                let mut chars: Vec<char> = self.cursor.take_while(is_digit);

                chars.insert(0, char);

                let int_str = String::from_iter(&chars);
                IntegerLiteral(int_str)
            }
            '+' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    PlusEquals
                }
                _ => Plus,
            },
            '-' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    MinusEquals
                }
                Some((_, '>')) => {
                    self.cursor.bump()?;
                    RArrow
                }
                _ => Minus,
            },
            '*' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    StarEquals
                }
                Some((_, '*')) => {
                    self.cursor.bump()?;

                    match self.cursor.chars.peek() {
                        Some((_, '=')) => {
                            self.cursor.bump()?;
                            StarStarEquals
                        }
                        _ => StarStar,
                    }
                }
                _ => Star,
            },
            '/' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
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
            '%' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    PercentEquals
                }
                _ => Percent,
            },
            '^' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    CaretEquals
                }
                _ => Caret,
            },
            '&' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    AmperEquals
                }
                _ => Amper,
            },
            '|' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    PipeEquals
                }
                _ => Pipe,
            },
            '~' => Tilde,
            '=' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    EqualsEquals
                }
                _ => Equals,
            },
            '<' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    LessEquals
                }
                Some((_, '<')) => {
                    self.cursor.bump()?;

                    match self.cursor.chars.peek() {
                        Some((_, '=')) => {
                            self.cursor.bump()?;
                            LShiftEquals
                        }
                        _ => LShift,
                    }
                }
                _ => Less,
            },
            '>' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
                    GreaterEquals
                }
                Some((_, '>')) => {
                    self.cursor.bump()?;

                    match self.cursor.chars.peek() {
                        Some((_, '=')) => {
                            self.cursor.bump()?;
                            RShiftEquals
                        }
                        _ => RShift,
                    }
                }
                _ => Greater,
            },
            '!' => match self.cursor.chars.peek() {
                Some((_, '=')) => {
                    self.cursor.bump()?;
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

        let span = Span::new(start_idx, self.cursor.idx);

        if self.cursor.chars.peek().is_none() {
            self.token_queue.push_back(Token {
                kind: NewLine,
                span: Span::new(self.cursor.idx, self.cursor.idx + 1),
            });

            for _ in 0..self.indentation_stack.len() - 1 {
                self.token_queue.push_back(Token {
                    kind: Dedent,
                    span: Span::new(self.cursor.idx + 1, self.cursor.idx + 1),
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
