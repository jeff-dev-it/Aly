mod number {
    use regex::Regex;

    pub fn is_any_number(item: &str) -> bool {
        is_int(item) || is_float(item)
    }

    pub fn is_int(item: &str) -> bool {
        let re = Regex::new(r"^[0-9]+$").unwrap();

        re.is_match(item)
    }

    pub fn is_float(item: &str) -> bool {
        let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();

        re.is_match(item)
    }

}

pub use number::*;