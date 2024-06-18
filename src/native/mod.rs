pub mod fs;
pub mod vars;
pub mod types;

mod native {    
    pub fn catch_error(_e: String){

    }
}

pub use native::*;