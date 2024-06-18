pub mod str;
pub mod numeric;
pub mod path;
pub mod structures;

mod validators {
    use regex::Regex;


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