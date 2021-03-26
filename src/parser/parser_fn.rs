//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;

/// A trait for parsers.
/// 
/// A parser is a stateful computation which given some input produces a value and a new
/// input. In the case of most parsers the input will be some sequence of values where
/// some prefix is consumed to produce the value and the unused suffix is returned as the
/// new state.
pub trait ParserFn<Input,> {
  /// The output produced by the parser.
  type Output;

  /// Parses `input`.
  fn call_parser(&self, input: Input,) -> Parse<Self::Output, Input,>;
}

impl<I, P,> ParserFn<I,> for &'_ P
  where P: ParserFn<I,> + ?Sized, {
  type Output = P::Output;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> { P::call_parser(self, input,) }
}

impl<I, P,> ParserFn<I,> for &'_ mut P
  where P: ParserFn<I,> + ?Sized, {
  type Output = P::Output;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> { P::call_parser(self, input,) }
}

impl<T, I,> ParserFn<I,> for fn(I,) -> Parse<T, I,> {
  type Output = T;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> { (self)(input,) }
}

impl<T, I,> ParserFn<I,> for dyn Fn(I,) -> Parse<T, I,> {
  type Output = T;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> { (self)(input,) }
}

impl<I,> ParserFn<I,> for ! {
  type Output = !;

  #[inline]
  fn call_parser(&self, _input: I,) -> Parse<Self::Output, I,> { *self }
}

#[cfg(feature = "alloc",)]
impl<I, P, A,> ParserFn<I,> for alloc::boxed::Box<P, A>
  where P: ParserFn<I,> + ?Sized,
    A: alloc::alloc::Allocator, {
  type Output = P::Output;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> { P::call_parser(self, input,) }
}

impl<I, P,> ParserFn<I,> for Option<P,>
  where P: ParserFn<I,>, {
  type Output = Option<P::Output>;

  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> {
    match self {
      Some(parser) => parser.call_parser(input,).map(Some,),
      None => Parse { value: None, unused: input, },
    }
  }
}

impl<I, A, B,> ParserFn<I,> for Result<A, B,>
  where A: ParserFn<I,>,
    B: ParserFn<I, Output = A::Output,>, {
  type Output = A::Output;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> {
    match self {
      Ok(parser) => parser.call_parser(input,),
      Err(parser) => parser.call_parser(input,),
    }
  }
}
