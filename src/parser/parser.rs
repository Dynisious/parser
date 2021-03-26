//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use super::*;
use crate::*;
use core::{
  ops::CoerceUnsized,
  convert::{AsRef, AsMut,},
};

/// An adaptor for parsers which provideds const methods for chaining and transforming
/// parsers.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Parser<P,>(pub P,)
  where P: ?Sized,;

impl<P,> Parser<P,> {
  /// Constructs a new `Parser` from `parser`.
  #[inline]
  pub const fn new(parser: P,) -> Self { Parser(parser,) }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> P {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const P) }
  }
}

impl<P,> Parser<P,>
  where P: ?Sized, {
  /// Parses `input` using the internal parser.
  pub fn parse<I, K,>(&self, input: K,) -> Parse<P::Output, I,>
    where K: Into<I> + ?Sized,
      P: ParserFn<I,>, { P::call_parser(&self.0, input.into(),) }
}

impl<I, P,> ParserFn<I,> for Parser<P,>
  where P: ParserFn<I,> + ?Sized, {
  type Output = P::Output;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> { P::call_parser(&self.0, input,) }
}

impl<P,> From<P> for Parser<P,> {
  #[inline]
  fn from(from: P,) -> Self { Parser(from,) }
}

impl<P,> AsRef<P> for Parser<P,>
  where P: ?Sized, {
  #[inline]
  fn as_ref(&self,) -> &P { &self.0 }
}

impl<P,> AsMut<P> for Parser<P,>
  where P: ?Sized, {
  #[inline]
  fn as_mut(&mut self,) -> &mut P { &mut self.0 }
}

impl<T, U,> CoerceUnsized<Parser<U,>> for Parser<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Parser<&i32,>,) {
  let _: Parser<&dyn Send> = a;
}
