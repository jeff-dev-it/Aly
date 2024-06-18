pub mod str;
pub mod numeric;
pub mod path;
pub mod structures;
pub mod reference;

mod validators {
    use regex::Regex;

    use super::{numeric::is_any_number, str::is_any_str};

    pub fn is_any_value(item: &str) -> bool {
        is_any_number(item) ||
        is_any_str(item) || 
        is_bool(item)
    }   

    pub fn is_bool(item: &str) -> bool {
        item == "true" || item == "false"
    } 

    pub fn is_char(item: &str) -> bool {
        let re = Regex::new("(?i)[a-z]").unwrap();

        re.is_match(item)
    }

    pub fn is_num(item: &str) -> bool {
        let re = Regex::new("[0-9]").unwrap();

        re.is_match(item)
    }

}

pub use validators::*;