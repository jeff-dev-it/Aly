
mod vars {
    use crate::native::types::{Type, ValueData};

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
}

pub use vars::*;