mod parser {
    use crate::{tokens::{get_token, Tokens}, validators::{is_char, is_num, structures::{is_close, is_opened, open_str}}};

    pub fn get_lexer(lines: Vec<&str>) {
        // let lexers = vec![];
        let mut to_end = 0;
        let mut is_str = Tokens::None;
        let mut ind = 1;

        for line in lines {
            let mut exp = String::new();
            let mut previous = "";

            for letter in line.split("") {
                exp.push_str(
                    &letter_per_letter(letter, previous, &mut to_end, &mut is_str)
                );

                previous = letter;
            }
            
            exp = exp.replace("  ", " ");

            ind += 1;

            println!("exp {exp}")
        }
    }

    fn letter_per_letter(letter: &str, previous: &str, to_end: &mut i32, is_str: &mut Tokens) -> String {
        let res = match letter.trim() {
            "" | " " => letter.to_string(),
            _ => {
                let tk = get_token(letter.to_owned());

                if is_char(letter) {
                    if is_char(previous) || previous == " " {
                        letter.to_string()
                    } else if open_str(Tokens::None, Some(is_str.clone())){
                        letter.to_string()
                    } else {
                        format!(" {}", letter)
                    }
                } else if is_num(letter) {
                    if is_num(previous) || previous == " " {
                        letter.to_string()
                    } else {
                        format!(" {}", letter)
                    }
                } else if open_str(Tokens::None, Some(is_str.clone())){
                    letter.to_string()
                } else {

                    if tk.id() == "identifier" {
                        return format!(" {} ", letter);
                    }
    
                    if is_opened(tk.clone()) {
                        *to_end += 1;
                        *is_str = tk.clone();
                    } else if is_close(tk.clone()) {
                        *to_end -= 1;
                        *is_str = Tokens::None;
                    }
    
                    if open_str(tk.clone(), Some(is_str.clone())) {
                        *is_str = tk.clone();
                        format!(" {}", letter)
                    } else {
                        *is_str = Tokens::None;
                        letter.to_string()
                    }
                }
            }
        };
    
        res
    }
    }

pub use parser::*;