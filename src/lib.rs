//! A library of parser combinators.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

#![no_std]
#![deny(missing_docs,)]
#![feature(
  never_type, coerce_unsized, try_trait, const_ptr_read, const_maybe_uninit_as_ptr,
  const_refs_to_cell, allocator_api, external_doc,
)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[macro_use]
extern crate std;

pub mod result;
pub mod parser;

pub use self::{
  result::{Parse, PResult,},
  parser::{Parser, ParserFn,},
};

#[cfg(doctest,)]
#[doc(include="../README.md",)]
struct DoctestReadme;
