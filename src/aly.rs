
mod aly {
    use std::env;

    use crate::{native::{fs::read_file, types::ValueData, vars::*}, runtime::parser::get_lexer, Act};
    
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

            get_lexer(codes);
        }

        // Variable manager
        pub fn get_vars(&self) -> &Vec<Var> {
            &self.datas
        }

        pub fn create_variable(&self) {

        }
    }
}

pub use aly::*;