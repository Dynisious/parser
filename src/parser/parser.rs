//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use super::{*, sources::*,};
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
  pub fn parse<I,>(&self, input: I,) -> Parse<P::Output, I,>
    where P: ParserFn<I,>, { P::call_parser(&self.0, input.into(),) }
}

impl Parser<Eof,> {
  /// The `Eof` parser.
  pub const EOF: Self = Parser(Eof,);
}

impl<T,> Parser<Always<T,>,> {
  /// Constructs a new `Parser` which always outputs `value`.
  /// 
  /// # Examples
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(42);
  /// assert_eq!(parser.parse("abc"), (42, "abc"));
  /// ```
  #[inline]
  pub const fn always(value: T,) -> Self { Self::new(Always::new(value,),) }
}

impl Parser<Next,> {
  /// Constructs a new parser which accepts the next `count` tokens.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::next(3);
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(&b"abc"[..]), &b""[..]));
  /// ```
  #[inline]
  pub const fn next(count: usize,) -> Self { Self::new(Next::new(count,),) }
}

impl<const N: usize,> Parser<NextN<N,>,> {
  /// Constructs a new parser which accepts the next `N` tokens.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::nextn();
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(b"abc"), &b""[..]));
  /// ```
  #[inline]
  pub const fn nextn() -> Self { Self::new(NextN,) }
}

impl<T,> Parser<Tag<T,>,> {
  /// Constructs a new parser which accepts a specific sequence of tokens.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::tag("abc");
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(&b"abc"[..]), &b""[..]));
  /// ```
  #[inline]
  pub const fn tag(tag: T,) -> Self { Self::new(Tag::new(tag,),) }
}

impl<T,> Parser<OneOf<T,>,> {
  /// Constructs a new parser which accepts the next token using a set of allowed tokens.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::one_of("ad");
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(b"a"), &b"bc"[..]));
  /// assert_eq!(parser.parse(&b"dbc"[..]), (Output(b"d"), &b"bc"[..]));
  /// ```
  #[inline]
  pub const fn one_of(one_of: T,) -> Self { Self::new(OneOf::new(one_of,),) }
}

impl<T,> Parser<NoneOf<T,>,> {
  /// Constructs a new parser which accepts the next token using a set of forbidden tokens.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::none_of("ab");
  /// assert_eq!(parser.parse(&b"abc"[..]), (Failed(b"a"), &b"abc"[..]));
  /// assert_eq!(parser.parse(&b"dbc"[..]), (Output(b"d"), &b"bc"[..]));
  /// ```
  #[inline]
  pub const fn none_of(none_of: T,) -> Self { Self::new(NoneOf::new(none_of,),) }
}

impl<F,> Parser<Sat<F,>,> {
  /// Constructs a new parser which accepts tokens as long as they satisfy a predicate.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::sat(|_, &t| t != b'c');
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(&b"ab"[..]), &b"c"[..]));
  /// assert_eq!(parser.parse(&b"c"[..]), (Output(&b""[..]), &b"c"[..]));
  /// ```
  #[inline]
  pub const fn sat(pred: F,) -> Self { Self::new(Sat::new(pred,),) }
}

impl<F,> Parser<Sat1<F,>,> {
  /// Constructs a new parser which accepts tokens as long as they satisfy a predicate.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::sat1(|_, &t| t != b'c');
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(&b"ab"[..]), &b"c"[..]));
  /// assert_eq!(parser.parse(&b"c"[..]), (Failed(b"c"), &b"c"[..]));
  /// ```
  #[inline]
  pub const fn sat1(pred: F,) -> Self { Self::new(Sat1::new(pred,),) }
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
