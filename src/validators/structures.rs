mod structures {
    use crate::tokens::Tokens;

    pub fn open_str(tk: Tokens, opened_one: Option<Tokens>) -> bool {
        if let Some(tk_opened) = opened_one {
            match (tk, tk_opened) {
                (Tokens::SimpleQuote, Tokens::SimpleQuote) |
                (Tokens::DupleQuote, Tokens::DupleQuote) => false,
                (Tokens::SimpleQuote, _) |
                (Tokens::DupleQuote, _) => true,
                (_, Tokens::SimpleQuote) |
                (_, Tokens::DupleQuote) => true,
                _ => false,
            }
        } else {
            match tk {
                Tokens::SimpleQuote | Tokens::DupleQuote => true,
                _ => false,
            }
        }
    }
    
    pub fn is_opened(tk: Tokens) -> bool{
        match tk {
            Tokens::LeftBrace |
            Tokens::LeftParenthesis | 
            Tokens::LeftBracket => true,
            _ => false
        }
    }

    pub fn is_close(tk: Tokens) -> bool{
        match tk {
            Tokens::RightBrace |
            Tokens::RightParenthesis | 
            Tokens::RightBracket => true,
            _ => false
        }
    }
}

pub use structures::*;