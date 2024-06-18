mod lexer {
    use crate::tokens::Tokens;

    pub struct Lexer {
        token: Tokens,
        literal: String,
        line: i32
    }

    impl Lexer {
        pub fn new(tk: Tokens, literal: String, line: i32) -> Lexer {
            Lexer {
                token: tk,
                line,
                literal: literal.trim().to_owned()
            }
        }

        pub fn as_string(&self) -> String {
            format!("===Lexer=token: {}, literal: {}, line: {}===", self.token, self.literal, self.line)
        }
    }
}

pub use lexer::*;