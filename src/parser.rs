//! Defines the [`Parser`](self::Parser) struct and [`ParserFn`](self::ParserFn) trait.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-04-01

mod parser_fn;
pub mod sources;
pub mod mapping;
pub mod sequence;
mod parser;

pub use self::{parser::*, parser_fn::*,};
