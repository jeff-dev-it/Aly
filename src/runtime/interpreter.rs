mod interpreter {
    use crate::{aly::Aly, lexer::Lexer, native::vars::{is_const_declaration, is_var_declaration}};

    pub fn exec(run: &mut Aly, lexers: &mut Vec<Lexer>){
        let mut to_made = "none";
        let mut ind = 0;
        let mut previous: Vec<Lexer> = vec![];

        for lex in &mut *lexers {
            if is_var_declaration(lex.token.clone()) {
                if (ind > 0 && to_made == "var_dec" && is_var_declaration(previous[ind - 1].clone().token)) || ind == 0 {
                    to_made = "var_dec";
                }
            } else if is_const_declaration(lex.token.clone()) {
                if (ind > 0 && to_made == "const_dec" && is_const_declaration(previous[ind - 1].clone().token)) || ind == 0 {
                    to_made = "const_dec";
                }
            }

            previous.push(lex.clone());
            ind += 1;
        }


        match to_made {
            "var_dec" => {
                run.create_variable(previous);
            }
            _ => (),
        }


        lexers.clear();
    }
}

pub use interpreter::*;