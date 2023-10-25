#[cfg(test)]
mod test {
    use crate::lexer::TokenKind as tk;
    use crate::lexer::{Lexer, Token, TokenKind};

    fn compare_tokens(expected_kinds: &[TokenKind], source_code: &str) {
        let lexer = Lexer::new(&source_code);

        let output_toks: Vec<Token> = lexer.collect();

        for i in 0..expected_kinds.len() {
            if output_toks.len() <= i {
                panic!(
                    "wrong len. expected={}, got={}",
                    expected_kinds.len(),
                    output_toks.len(),
                );
            }

            println!("{:?} -- {:?}", expected_kinds[i], output_toks[i].kind);

            assert_eq!(
                expected_kinds[i], output_toks[i].kind,
                "tests[{}] - token kind wrong. expected={:?}, got={:?}",
                i, expected_kinds[i], output_toks[i].kind,
            );
        }

        assert_eq!(
            expected_kinds.len(),
            output_toks.len(),
            "wrong len. expected={}, got={}",
            expected_kinds.len(),
            output_toks.len(),
        );
    }

    // TODO: Add test function for testing spans and not just TokenKind

    #[test]
    fn math_tokens() {
        let src = "+-*/";
        let kinds = &[tk::Plus, tk::Minus, tk::Star, tk::Slash, tk::NewLine];

        compare_tokens(kinds, src);
    }

    #[test]
    fn one_char_tokens() {
        let src = "()[]{}:;.,+-*/%^&|~=<>!@";
        let kinds = &[
            tk::LParen,
            tk::RParen,
            tk::LBracket,
            tk::RBracket,
            tk::LBrace,
            tk::RBrace,
            tk::Colon,
            tk::Semi,
            tk::Dot,
            tk::Comma,
            tk::Plus,
            tk::Minus,
            tk::Star,
            tk::Slash,
            tk::Percent,
            tk::Caret,
            tk::Amper,
            tk::Pipe,
            tk::Tilde,
            tk::Equals,
            tk::Less,
            tk::Greater,
            tk::Not,
            tk::At,
            tk::NewLine,
        ];

        compare_tokens(kinds, src);
    }

    #[test]
    fn multi_char_tokens() {
        let src = "-> == != <= >= << >> ** += -= *= /= %= &= |= ^= <<= >>= **=";
        let kinds = &[
            tk::RArrow,
            tk::EqualsEquals,
            tk::NotEquals,
            tk::LessEquals,
            tk::GreaterEquals,
            tk::LShift,
            tk::RShift,
            tk::StarStar,
            tk::PlusEquals,
            tk::MinusEquals,
            tk::StarEquals,
            tk::SlashEquals,
            tk::PercentEquals,
            tk::AmperEquals,
            tk::PipeEquals,
            tk::CaretEquals,
            tk::LShiftEquals,
            tk::RShiftEquals,
            tk::StarStarEquals,
            tk::NewLine,
        ];

        compare_tokens(kinds, src);
    }

    #[test]
    fn basic_program() {
        let src = r#"one = 1
print(one)"#;
        let kinds = &[
            tk::Identifier("one".into()),
            tk::Equals,
            tk::IntegerLiteral("1".into()),
            tk::NewLine,
            tk::Identifier("print".into()),
            tk::LParen,
            tk::Identifier("one".into()),
            tk::RParen,
            tk::NewLine,
        ];

        compare_tokens(kinds, src);
    }

    #[test]
    fn normal_indent() {
        let src = r#"+
    +
+
    +"#;

        let kinds = &[
            tk::Plus,
            tk::NewLine,
            tk::Indent,
            tk::Plus,
            tk::NewLine,
            tk::Dedent,
            tk::Plus,
            tk::NewLine,
            tk::Indent,
            tk::Plus,
            tk::NewLine,
            tk::Dedent,
        ];

        compare_tokens(kinds, src);
    }

    #[test]
    fn basic_func_def() {
        let src = r#"def hello():
    print("Hello world!")"#;

        let kinds = &[
            tk::Identifier("def".into()),
            tk::Identifier("hello".into()),
            tk::LParen,
            tk::RParen,
            tk::Colon,
            tk::NewLine,
            tk::Indent,
            tk::Identifier("print".into()),
            tk::LParen,
            tk::StringLiteral("Hello world!".into()),
            tk::RParen,
            tk::NewLine,
            tk::Dedent,
        ];

        compare_tokens(kinds, src);
    }

    #[test]
    fn indent_twice() {
        let src = r#"def print_if_five(number: int):
    if number == 5:
        print("It is five!")"#;

        let kinds = &[
            tk::Identifier("def".into()),
            tk::Identifier("print_if_five".into()),
            tk::LParen,
            tk::Identifier("number".into()),
            tk::Colon,
            tk::Identifier("int".into()),
            tk::RParen,
            tk::Colon,
            tk::NewLine,
            tk::Indent,
            tk::Identifier("if".into()),
            tk::Identifier("number".into()),
            tk::EqualsEquals,
            tk::IntegerLiteral("5".into()),
            tk::Colon,
            tk::NewLine,
            tk::Indent,
            tk::Identifier("print".into()),
            tk::LParen,
            tk::StringLiteral("It is five!".into()),
            tk::RParen,
            tk::NewLine,
            tk::Dedent,
            tk::Dedent,
        ];

        compare_tokens(kinds, src);
    }

    #[test]
    fn valid_ident_names() {
        let src = r#"valid _valid __valid v4l1d valid_als0 1nvalid"#;

        let kinds = &[
            tk::Identifier("valid".into()),
            tk::Identifier("_valid".into()),
            tk::Identifier("__valid".into()),
            tk::Identifier("v4l1d".into()),
            tk::Identifier("valid_als0".into()),
            tk::Unknown,
        ];

        compare_tokens(kinds, src);
    }
}
