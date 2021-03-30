//! Transformers of the output type of a parser by sequencing multiple parsers.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-30

use crate::*;
use core::ops::{Try, CoerceUnsized,};

/// A parser which maps the output of a parser to a one-time-parse function which is then
/// applied to the remaining input.
#[derive(Clone, Copy, Default, Debug,)]
pub struct Seq<F, P,>
  where P: ?Sized, {
  /// The mapping to apply.
  map: F,
  /// The parser to map.
  parser: P,
}

impl<F, P,> Seq<F, P,> {
  /// Constructs a new `Seq` using `map` and `parser`.
  #[inline]
  pub const fn new(map: F, parser: P,) -> Self { Seq { map, parser, } }
}

impl<F, P, T, I,> ParserFn<I,> for Seq<F, P,>
  where F: Fn<(P::Output,)>,
    P: ParserFn<I,>,
    F::Output: FnOnce(I,) -> Parse<T, I,>, {
  type Output = T;

  #[inline]
  fn parse(&self, input: I,) -> Parse<Self::Output, I,> {
    let Parse { value, unused, } = self.parser.parse(input,);
    (self.map)(value,)(unused,)
  }
}

impl<F, T, U,> CoerceUnsized<Seq<F, U,>> for Seq<F, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A parser which maps the successful output of a parser to a one-time-parse function
/// which is then applied to the remaining input.
/// 
/// If either output errors the error is returned with the original input.
#[derive(Clone, Copy, Default, Debug,)]
pub struct TrySeq<F, P,>
  where P: ?Sized, {
  /// The mapping to apply.
  map: F,
  /// The parser to map.
  parser: P,
}

impl<F, P,> TrySeq<F, P,> {
  /// Constructs a new `TrySeq` using `map` and `parser`.
  #[inline]
  pub const fn new(map: F, parser: P,) -> Self { TrySeq { map, parser, } }
}

impl<F, P, T, I,> ParserFn<I,> for TrySeq<F, P,>
  where F: Fn<(<P::Output as Try>::Ok,)>,
    P: ParserFn<I,>,
    T: Try,
    F::Output: FnOnce(I,) -> Parse<T, I,>,
    P::Output: Try,
    T::Error: From<<P::Output as Try>::Error>, {
  type Output = T;

  fn parse(&self, input: I,) -> Parse<Self::Output, I,> {
    let Parse { value, unused, } = self.parser.parse(input,)
      .into_result()
      .map_err(|e,| e.map(From::from,),)?;
    (self.map)(value,)(unused,)
  }
}

impl<F, T, U,> CoerceUnsized<TrySeq<F, U,>> for TrySeq<F, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A parser which maps the failure output of a parser to a one-time-parse function
/// which is then applied to the original input.
/// 
/// If either output errors the error is returned.
#[derive(Clone, Copy, Default, Debug,)]
pub struct TrySeqErr<F, P,>
  where P: ?Sized, {
  /// The mapping to apply.
  map: F,
  /// The parser to map.
  parser: P,
}

impl<F, P,> TrySeqErr<F, P,> {
  /// Constructs a new `TrySeqErr` using `map` and `parser`.
  #[inline]
  pub const fn new(map: F, parser: P,) -> Self { TrySeqErr { map, parser, } }
}

impl<F, P, T, I,> ParserFn<I,> for TrySeqErr<F, P,>
  where F: Fn<(<P::Output as Try>::Error,)>,
    P: ParserFn<I,>,
    T: Try,
    F::Output: FnOnce(I,) -> Parse<T, I,>,
    P::Output: Try,
    T::Ok: From<<P::Output as Try>::Ok>, {
  type Output = T;

  fn parse(&self, input: I,) -> Parse<Self::Output, I,> {
    let Parse { value, unused, } = match self.parser.parse(input,).into_result() {
      Ok(parse) => return Parse::from_ok(parse.map(From::from,),),
      Err(parse) => parse,
    };
    (self.map)(value,)(unused,)
  }
}

impl<F, T, U,> CoerceUnsized<TrySeqErr<F, U,>> for TrySeqErr<F, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

fn _assert_coerce_unsized(a: Seq<(), &i32,>, b: TrySeq<(), &i32,>, c: TrySeqErr<(), &i32,>,) {
  let _: Seq<(), &dyn Send,> = a;
  let _: TrySeq<(), &dyn Send,> = b;
  let _: TrySeqErr<(), &dyn Send,> = c;
}
