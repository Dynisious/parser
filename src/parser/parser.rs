//! Author --- DMorgan  
//! Last Moddified --- 2021-03-30

use super::{*, sources::*, mapping::*, apply::*, sequence::*,};
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
  /// References the inner value.
  #[inline]
  pub const fn as_ref(&self,) -> Parser<&P,> { Parser(&self.0,) }
  /// Mutably references the inner value.
  #[inline]
  pub const fn as_mut(&mut self,) -> Parser<&mut P,> { Parser(&mut self.0,) }
  /// Parses `input` using the internal parser.
  pub fn parse<I,>(&self, input: I,) -> Parse<P::Output, I,>
    where P: ParserFn<I,>, { P::parse(&self.0, input.into(),) }
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
  pub const fn tag<I,>(tag: T,) -> Self
    where Self: ParserFn<I,>, { Self::new(Tag::new(tag,),) }
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
  pub const fn one_of<I,>(one_of: T,) -> Self
    where Self: ParserFn<I,>, { Self::new(OneOf::new(one_of,),) }
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
  pub const fn none_of<I,>(none_of: T,) -> Self
    where Self: ParserFn<I,>, { Self::new(NoneOf::new(none_of,),) }
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
  pub const fn sat<I,>(pred: F,) -> Self
    where Self: ParserFn<I,>, { Self::new(Sat::new(pred,),) }
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
  pub const fn sat1<I,>(pred: F,) -> Self
    where Self: ParserFn<I,>, { Self::new(Sat1::new(pred,),) }
}

impl<P,> Parser<P,> {
  /// Maps the output type of the inner parser using `map`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(21).map::<&str, _>(|x| x * 2);
  /// assert_eq!(parser.parse("abc"), (42, "abc"));
  /// ```
  #[inline]
  pub const fn map<I, F,>(self, map: F,) -> Parser<Map<F, P,>,>
    where Map<F, P,>: ParserFn<I,>, { Parser::new(Map::new(map, self.into_inner(),),) }
  /// Applies the output of this parser to the output of `value`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(|x| x * 2).apply::<&str, _>(Parser::always(21));
  /// assert_eq!(parser.parse("abc"), (42, "abc"));
  /// ```
  #[inline]
  pub const fn apply<I, T,>(self, value: T,) -> Parser<Apply<P, T,>,>
    where Apply<P, T,>: ParserFn<I,>, { Parser::new(Apply::new(self.into_inner(), value,),) }
  /// Applies the successful output of this parser to the successful output of `value`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Ok(|x| Ok(x * 2))).try_apply::<&str, _>(Parser::always(Ok(21)));
  /// assert_eq!(parser.parse("abc"), (Ok(42) as Result<i32, ()>, "abc"));
  /// ```
  #[inline]
  pub const fn try_apply<I, T,>(self, value: T,) -> Parser<TryApply<P, T,>,>
    where TryApply<P, T,>: ParserFn<I,>, { Parser::new(TryApply::new(self.into_inner(), value,),) }
  /// Applies the failure output of this parser to the failure output of `value`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Err(|x| Err(x * 2))).try_apply_err::<&str, _>(Parser::always(Err(21)));
  /// assert_eq!(parser.parse("abc"), (Err(42) as Result<(), i32>, "abc"));
  /// ```
  #[inline]
  pub const fn try_apply_err<I, T,>(self, value: T,) -> Parser<TryApplyErr<P, T,>,>
    where TryApplyErr<P, T,>: ParserFn<I,>, { Parser::new(TryApplyErr::new(self.into_inner(), value,),) }
  /// Maps the output of this parser to a one-time-parse function which is applied to the
  /// unused input.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(21).seq::<&str, _>(|x| Parser::always(x * 2));
  /// assert_eq!(parser.parse("abc"), (42, "abc"));
  /// ```
  #[inline]
  pub const fn seq<I, F,>(self, map: F,) -> Parser<Seq<F, P,>,>
    where Seq<F, P,>: ParserFn<I,>, { Parser::new(Seq::new(map, self.into_inner(),),) }
  /// Maps the successful output of this parser to a one-time-parse function which is
  /// applied to the unused input.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Ok(21)).try_seq::<&str, _>(|x| Parser::always(Ok(x * 2)));
  /// assert_eq!(parser.parse("abc"), (Ok(42) as Result<i32, ()>, "abc"));
  /// ```
  #[inline]
  pub const fn try_seq<I, F,>(self, map: F,) -> Parser<TrySeq<F, P,>,>
    where TrySeq<F, P,>: ParserFn<I,>, { Parser::new(TrySeq::new(map, self.into_inner(),),) }
  /// Maps the failed output of this parser to a one-time-parse function which is applied
  /// to the unused input.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Err(21)).try_seq_err::<&str, _>(|x| Parser::always(Err(x * 2)));
  /// assert_eq!(parser.parse("abc"), (Err(42) as Result<(), i32>, "abc"));
  /// ```
  #[inline]
  pub const fn try_seq_err<I, F,>(self, map: F,) -> Parser<TrySeqErr<F, P,>,>
    where TrySeqErr<F, P,>: ParserFn<I,>, { Parser::new(TrySeqErr::new(map, self.into_inner(),),) }
}

impl<I, P,> ParserFn<I,> for Parser<P,>
  where P: ParserFn<I,> + ?Sized, {
  type Output = P::Output;

  #[inline]
  fn parse(&self, input: I,) -> Parse<Self::Output, I,> { P::parse(&self.0, input,) }
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

impl<I, P,> FnOnce<(I,)> for Parser<P,>
  where P: ParserFn<I,>, {
  type Output = Parse<P::Output, I,>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (I,),) -> Self::Output { self.parse(input,) }
}

impl<I, P,> FnMut<(I,)> for Parser<P,>
  where P: ParserFn<I,>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (I,),) -> Self::Output { self.parse(input,) }
}

impl<I, P,> Fn<(I,)> for Parser<P,>
  where P: ParserFn<I,>, {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (I,),) -> Self::Output { self.parse(input,) }
}

impl<T, U,> CoerceUnsized<Parser<U,>> for Parser<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

#[allow(unused,)]
fn _assert_coerce_unsized(a: Parser<&i32,>,) {
  let _: Parser<&dyn Send> = a;
}
