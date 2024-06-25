mod types {
    use core::fmt;

    use crate::{aly::Aly, lexer::Lexer, validators::{conversor_to_bool, conversor_to_float, conversor_to_int, is_bool, numeric::{is_float, is_int}, str::{is_any_str, put_quoted_str, remove_quoted_str}}};

    pub enum Type {
        Int,
        Float,
        String,
        Bool,
        Vec,
        Obj,
        None,
        Struct(String),
        Model(String),
        Function,   
        NativeFunction,
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let res = match self {
                Type::Int => "int",
                Type::Float => "float",
                Type::String => "string",
                Type::Bool => "boolean",
                Type::Vec => "vector",
                Type::None => "None",
                Type::Obj => "obj",
                Type::Struct(_) => "Struct",
                Type::Model(_) => "Model",
                Type::Function => "Function",
                Type::NativeFunction => "NativeFunction",
            };

            write!(f, "{}", res)
        }
    }

    pub enum ValueData {
        Int(i32),
        Float(f32),
        String(String),
        Bool(bool),
        Vec(Vec<String>),  
        Function(Vec<Lexer>),   
        NativeFunction(fn(&mut Aly, String) -> Box<dyn Validator>),
    }

    impl ValueData {
        pub fn to_string(&self, qt: bool) -> String {
            match self {
                ValueData::Int(int) => int.to_string(),
                ValueData::Float(f) => f.to_string(),
                ValueData::String(s) => {
                    if qt {
                        put_quoted_str(s.to_string())
                    } else {
                        s.to_string()
                    }
                },
                ValueData::Bool(_) |
                ValueData::Vec(_) => String::from("None"),
                ValueData::Function(_) => "Function".to_owned(),
                ValueData::NativeFunction(_) => "NativeFunction".to_owned(),
            }
        }
    }

    impl Clone for ValueData {
        fn clone(&self) -> Self {
            match self {
                ValueData::Int(d) => ValueData::Int(d.clone()),
                ValueData::Float(d) => ValueData::Float(d.clone()),
                ValueData::String(d) => ValueData::String(d.clone()),
                ValueData::Bool(d) => ValueData::Bool(d.clone()),
                ValueData::Vec(d) => ValueData::Vec(d.clone()),
                ValueData::Function(fun) => ValueData::Function(fun.clone()),
                ValueData::NativeFunction(fun) => ValueData::NativeFunction(fun.clone()),
            }
        }
    }

    // Validator value
    
    pub trait Validator {
        fn valid(&self) -> (Type, ValueData);
    }

    impl Validator for () {
        fn valid(&self) -> (Type, ValueData) {
            (Type::None, ValueData::String("None".to_owned())) 
        }
    }

    
    impl Validator for fn(&mut Aly, String) -> Box<dyn Validator> {
        fn valid(&self) -> (Type, ValueData) {
            (Type::NativeFunction, ValueData::NativeFunction(*self)) 
        }
    }

    impl Validator for String {
        fn valid(&self) -> (Type, ValueData) {
            if is_any_str(self) {
                (Type::String, ValueData::String(remove_quoted_str(self.to_string()))) 
            } else if is_bool(&self) {
                (Type::Bool, ValueData::Bool(conversor_to_bool(self.to_string())))
            } else if is_int(self) {
                (Type::Int, ValueData::Int(conversor_to_int(self.to_string())))
            } else if is_float(self) {
                (Type::Int, ValueData::Float(conversor_to_float(self.to_string())))
            } else {
                (Type::None, ValueData::String("None".to_owned())) 
            }
        }
    }
    

    pub fn is_valid_data<T: Validator>(data: T) -> (Type, ValueData) {
        data.valid()
    }
}

pub use types::*;