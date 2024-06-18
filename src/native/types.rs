mod types {
    pub enum Type {
        Int,
        Float,
        String,
        Bool,
        Vec,
        Obj,
        Struct(String),
        Model(String),        
    }

    pub enum ValueData {
        Int(i32),
        Float(f32),
        String(String),
        Bool(bool),
        Vec(Vec<String>),     
    }

    impl ValueData {
        pub fn to_string(&self, _: bool) -> String {
            match self {
                ValueData::Int(int) => int.to_string(),
                ValueData::Float(f) => f.to_string(),
                ValueData::String(s) => s.to_string(),
                ValueData::Bool(_) |
                ValueData::Vec(_) => String::from("None"),
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
            }
        }
    }
}

pub use types::*;