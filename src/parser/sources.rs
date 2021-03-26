//! The inbuilt source parsers.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

mod eof;
mod always;
mod next;
mod tag;
mod one_of;
mod sat;

pub use self::{eof::*, always::*, next::*, tag::*, one_of::*, sat::*,};
