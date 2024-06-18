
mod vars {
    use crate::{native::types::{Type, ValueData}, tokens::Tokens};

    pub struct Var {
        name: String,
        value: ValueData,
        data_type: Type
    }

    impl Var {
        pub fn new(name: String, value: String) -> Var {
            Var { name, value: ValueData::String(value), data_type: Type::String }
        }

        pub fn compare_var(&self, name: String) -> bool {
            &self.name == name.trim()
        }

        pub fn get_value(&self) -> ValueData {
            self.value.clone()
        }
    }

    pub fn is_var_declaration(tk: Tokens) -> bool{
        match tk {
            Tokens::Let | 
            Tokens::Reference | 
            Tokens::Identifier | 
            Tokens::Value => true,
            _ => false
        }
    }

    pub fn is_const_declaration(tk: Tokens) -> bool{
        match tk {
            Tokens::Const | 
            Tokens::Reference | 
            Tokens::Identifier | 
            Tokens::Value => true,
            _ => false
        }
    }
}

pub use vars::*;