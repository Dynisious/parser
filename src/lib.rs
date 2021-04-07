//! A library of parser combinators.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-31

#![no_std]
#![deny(missing_docs,)]
#![feature(
  never_type, coerce_unsized, try_trait, const_ptr_read, const_maybe_uninit_as_ptr,
  const_refs_to_cell, allocator_api, external_doc, bool_to_option, array_from_ref,
  unboxed_closures, const_fn, const_mut_refs, fn_traits, const_fn_fn_ptr_basics,
  const_fn_transmute, const_raw_ptr_deref, const_panic, min_type_alias_impl_trait,
)]

#[cfg(feature = "alloc",)]
extern crate alloc;
#[cfg(any(test, doctest,),)]
#[macro_use]
extern crate std;

pub mod result;
pub mod parser;
pub mod combinators;

pub use self::{
  result::{Parse, PResult::{self, *,},},
  parser::{Parser, ParserFnOnce, ParserFnMut, ParserFn,},
};

#[cfg(doctest,)]
#[doc(include="../README.md",)]
struct DoctestReadme;
