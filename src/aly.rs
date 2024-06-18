
mod aly {
    use std::env;

    use crate::{lexer::Lexer, native::{fs::read_file, types::ValueData, vars::*}, runtime::parser::get_lexer, tokens::Tokens, Act};
    
    pub struct Aly {
        args: Vec<String>,
        action: Act,
        datas: Vec<Var>
    }

    impl Aly {
        pub fn new(action: Act) -> Aly {
            let cwd = match env::current_dir() {
                Err(why) => panic!("Erro ao iniciar o programa, {why}"),
                Ok(item) => item,
            };

            Aly {
                args: vec![],
                action,
                datas: vec![
                    Var::new(String::from("_dir_call"), cwd.display().to_string()),
                ]
            }
        }

        // Runtime
        pub fn run(&mut self, file: String){
            match self.action {
                Act::Run => self.run_code(file),
                Act::Cli => {},
                Act::Comp => {},
            }
        }
        // Internal Functions 
        fn run_code(&mut self, path: String) {
            let file_to_run = read_file(path);
            let codes: Vec<&str> = file_to_run.trim().split("\n").collect();

            get_lexer(self, codes);
        }

        // Variable manager
        pub fn get_vars(&self) -> &Vec<Var> {
            &self.datas
        }

        pub fn create_variable(&self, lexers: Vec<Lexer>) {
            let name = &lexers[1];

            if lexers.len() == 2 {
                println!("none");
                
                return;
            } 
            
            let identifier = &lexers[2];
            let value = &lexers[3];

            match identifier.token {
                Tokens::Identifier => (),
                _ => panic!("")
            };

            let var = Var::new(name.literal.to_string(), value.literal.to_string());

            println!("{var}")
        }
    }
}

pub use aly::*;