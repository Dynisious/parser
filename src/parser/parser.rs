//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use super::{*, sources::*, mapping::*, sequence::*,};
use crate::combinators::{
  Pair, And, AndOk, Or, TryMap, TryMapErr, SeqApply, SeqApplyOk, SeqApplyErr, SeqPipe,
  TrySeq, TrySeqErr,
};
use core::{
  ops::Try,
  convert::{AsRef, AsMut,},
};

/// An adaptor for parsers which provideds const methods for chaining and transforming
/// parsers.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Parser<P,>(pub P,);

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
  /// Maps the inner parser.
  #[inline]
  pub fn map_inner<Q, F,>(self, map: F,) -> Parser<Q,>
    where F: FnOnce(P,) -> Q, { Parser::new(map(self.into_inner(),),) }
  /// References the inner value.
  #[inline]
  pub const fn as_ref(&self,) -> Parser<&P,> { Parser(&self.0,) }
  /// Mutably references the inner value.
  #[inline]
  pub const fn as_mut(&mut self,) -> Parser<&mut P,> { Parser(&mut self.0,) }
}

impl<P,> Parser<&'_ P,> {
  /// Clones the internal parser.
  #[inline]
  pub fn cloned(&self,) -> Parser<P,>
    where P: Clone, { Parser(self.0.clone(),) }
  /// Copies the internal parser.
  #[inline]
  pub const fn copied(&self,) -> Parser<P,>
    where P: Copy, { Parser(*self.0,) }
}

impl<P,> Parser<&'_ mut P,> {
  /// Clones the internal parser.
  #[inline]
  pub fn cloned(&self,) -> Parser<P,>
    where P: Clone, { Parser(self.0.clone(),) }
  /// Copies the internal parser.
  #[inline]
  pub const fn copied(&self,) -> Parser<P,>
    where P: Copy, { Parser(*self.0,) }
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
    where Self: ParserFnOnce<I,>, { Self::new(Tag::new(tag,),) }
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
    where Self: ParserFnOnce<I,>, { Self::new(OneOf::new(one_of,),) }
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
    where Self: ParserFnOnce<I,>, { Self::new(NoneOf::new(none_of,),) }
}

impl<F,> Parser<Sat<F,>,> {
  /// Constructs a new parser which accepts tokens as long as they satisfy a predicate.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let pred = |_, &t| t != b'c';
  /// let parser = Parser::sat(&pred);
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(&b"ab"[..]), &b"c"[..]));
  /// assert_eq!(parser.parse(&b"c"[..]), (Output(&b""[..]), &b"c"[..]));
  /// ```
  #[inline]
  pub const fn sat<I,>(pred: F,) -> Self
    where Self: ParserFnOnce<I,>, { Self::new(Sat::new(pred,),) }
}

impl<F,> Parser<Sat1<F,>,> {
  /// Constructs a new parser which accepts tokens as long as they satisfy a predicate.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let pred = |_, &t| t != b'c';
  /// let parser = Parser::sat1(&pred);
  /// assert_eq!(parser.parse(&b"abc"[..]), (Output(&b"ab"[..]), &b"c"[..]));
  /// assert_eq!(parser.parse(&b"c"[..]), (Failed(b"c"), &b"c"[..]));
  /// ```
  #[inline]
  pub const fn sat1<I,>(pred: F,) -> Self
    where Self: ParserFnOnce<I,>, { Self::new(Sat1::new(pred,),) }
}

impl<P,> Parser<P,> {
  /// Maps the output type of the inner parser using `map`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let map = |x| x * 2;
  /// let parser = Parser::always(21).map::<&str, _>(&map);
  /// assert_eq!(parser.parse("abc"), (42, "abc"));
  /// ```
  #[inline]
  pub const fn map<I, F,>(self, map: F,) -> Parser<Map<F, P,>,>
    where Map<F, P,>: ParserFnOnce<I,>, { Parser::new(Map::new(map, self.into_inner(),),) }
  /// Maps the output successful output of the inner parser using `map`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let map = |x| x * 2;
  /// let parser = Parser::always(Ok(21)).map_ok::<&str, _>(map);
  /// assert_eq!(parser.parse("abc"), (Ok(42) as Result<i32, ()>, "abc"));
  /// ```
  #[inline]
  pub const fn map_ok<I, F,>(self, map: F,) -> Parser<MapOk<F, P,>,>
    where MapOk<F, P,>: ParserFnOnce<I,>, { self.map(TryMap::new(map,),) }
  /// Maps the output successful output of the inner parser using `map`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let map = |x| x * 2;
  /// let parser = Parser::always(Err(21)).map_err::<&str, _>(&map);
  /// assert_eq!(parser.parse("abc"), (Err(42) as Result<(), i32>, "abc"));
  /// ```
  #[inline]
  pub const fn map_err<I, F,>(self, map: F,) -> Parser<MapErr<F, P,>,>
    where MapErr<F, P,>: ParserFnOnce<I,>, { self.map(TryMapErr::new(map,),) }
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
    where Apply<P, T,>: ParserFnOnce<I,>, { self.pipe(SeqApply(value,),) }
  /// Applies the successful output of this parser to the successful output of `value`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Ok(|x| x * 2)).apply_ok::<&str, _>(Parser::always(Ok(21)));
  /// assert_eq!(parser.parse("abc"), (Ok(42) as Result<i32, ()>, "abc"));
  /// ```
  #[inline]
  pub const fn apply_ok<I, T,>(self, value: T,) -> Parser<ApplyOk<P, T,>,>
    where ApplyOk<P, T,>: ParserFnOnce<I,>, { self.pipe(SeqApplyOk(value,),) }
  /// Applies the failure output of this parser to the failure output of `value`.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Err(|x| x * 2)).apply_err::<&str, _>(Parser::always(Err(21)));
  /// assert_eq!(parser.parse("abc"), (Err(42) as Result<(), i32>, "abc"));
  /// ```
  #[inline]
  pub const fn apply_err<I, T,>(self, value: T,) -> Parser<ApplyErr<P, T,>,>
    where P: ParserFnOnce<I,>,
      ApplyErr<P, T,>: ParserFnOnce<I,>, { self.pipe(SeqApplyErr(value,),) }
  /// Pipes the output of this `Parser` through a mapping for `Parse`s.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let seq = |parse: Parse<_, _>| parse.map(|x| x * 2);
  /// let parser = Parser::always(21).pipe::<&str, _>(&seq);
  /// assert_eq!(parser.parse("abc"), (42, "abc"));
  /// ```
  #[inline]
  pub const fn pipe<I, F,>(self, map: F,) -> Parser<Pipe<F, P,>,>
    where Pipe<F, P,>: ParserFnOnce<I,>, { Parser::new(Pipe::new(map, self.into_inner(),),) }
  /// Maps the output of this parser to a one-time-parse function which is applied to the
  /// unused input.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let seq = |x| Parser::always(x * 2);
  /// let parser = Parser::always(21).seq::<&str, _>(&seq);
  /// assert_eq!(parser.parse("abc"), (42, "abc"));
  /// ```
  #[inline]
  pub const fn seq<I, F,>(self, map: F,) -> Parser<Seq<F, P,>,>
    where Seq<F, P,>: ParserFnOnce<I,>, { self.pipe(SeqPipe(map,),) }
  /// Maps the successful output of this parser to a one-time-parse function which is
  /// applied to the unused input.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let seq = |x| Parser::always(Ok(x * 2));
  /// let parser = Parser::always(Ok(21)).seq_ok::<&str, _>(&seq);
  /// assert_eq!(parser.parse("abc"), (Ok(42) as Result<i32, ()>, "abc"));
  /// ```
  #[inline]
  pub const fn seq_ok<I, F,>(self, map: F,) -> Parser<SeqOk<F, P,>,>
    where SeqOk<F, P,>: ParserFnOnce<I,>, { self.pipe(TrySeq(map,),) }
  /// Maps the failed output of this parser to a one-time-parse function which is applied
  /// to the unused input.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let seq = |x: i32| Parser::always(Err(x * 2));
  /// let parser = Parser::always(Err(21)).seq_err::<&str, _>(&seq);
  /// assert_eq!(parser.parse("abc"), (Err(42) as Result<(), i32>, "abc"));
  /// ```
  #[inline]
  pub const fn seq_err<I, F,>(self, map: F,) -> Parser<SeqErr<F, P,>,>
    where SeqErr<F, P,>: ParserFnOnce<I,>, { self.pipe(TrySeqErr(map,),) }
}

impl<P,> Parser<P,> {
  /// Sequences both of the parsers and returns both outputs in a tuple.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always('a').and::<&str, _>(Parser::always('b'));
  /// assert_eq!(parser.parse("abc"), (('a', 'b'), "abc"));
  /// ```
  #[inline]
  pub const fn and<I, Q,>(self, parser: Q,) -> Parser<And<P, Q,>,>
    where P: ParserFnOnce<I,>,
      And<P, Q,>: ParserFnOnce<I,>, { self.map(Pair,).apply(parser,) }
  /// Sequences both parsers and returns the successful output of both in a tuple.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Ok('a')).and_ok::<&str, _>(Parser::always(Ok('b')));
  /// assert_eq!(parser.parse("abc"), (Ok(('a', 'b')) as Result<(char, char), ()>, "abc"));
  /// ```
  #[inline]
  pub const fn and_ok<I, Q,>(self, parser: Q,) -> Parser<AndOk<P, Q,>,>
    where P: ParserFnOnce<I,>,
      P::Value: Try,
      AndOk<P, Q,>: ParserFnOnce<I,>, { self.map_ok(Pair,).apply_ok(parser,) }
  /// Returns the first successful output of either parser.
  /// 
  /// ```
  /// use ::parser::*;
  /// 
  /// let parser = Parser::always(Ok('a')).or::<&str, _>(Parser::always(Ok('b')));
  /// assert_eq!(parser.parse("abc"), (Ok('a') as Result<char, ((), ())>, "abc"));
  /// let parser = Parser::always(Err(()) as Result<char, _>).or::<&str, _>(Parser::always(Ok('b')));
  /// assert_eq!(parser.parse("abc"), (Ok('b') as Result<char, ((), ())>, "abc"));
  /// ```
  #[inline]
  pub const fn or<I, Q,>(self, parser: Q,) -> Parser<Or<P, Q,>,>
    where P: ParserFnOnce<I,>,
      P::Value: Try,
      Or<P, Q,>: ParserFnOnce<I,>, { self.map_err(Pair,).apply_err(parser,) }
}

impl<P,> From<P> for Parser<P,> {
  #[inline]
  fn from(from: P,) -> Self { Parser(from,) }
}

impl<P,> AsRef<P> for Parser<P,> {
  #[inline]
  fn as_ref(&self,) -> &P { &self.0 }
}

impl<P,> AsMut<P> for Parser<P,> {
  #[inline]
  fn as_mut(&mut self,) -> &mut P { &mut self.0 }
}

impl<I, P,> FnOnce<(I,)> for Parser<P,>
  where P: ParserFnOnce<I,>, {
  type Output = P::Output;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (I,),) -> Self::Output { (self.0)(input,) }
}

impl<I, P,> FnMut<(I,)> for Parser<P,>
  where P: ParserFnMut<I,>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (I,),) -> Self::Output { (self.0)(input,) }
}

impl<I, P,> Fn<(I,)> for Parser<P,>
  where P: ParserFn<I,>, {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (I,),) -> Self::Output { (self.0)(input,) }
}
