//! Transformers of the output type of a parser by mapping the output value.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;
use core::ops::CoerceUnsized;

/// A parser which maps the output value of the inner parser.
#[derive(Clone, Copy, Default, Debug,)]
pub struct Map<F, P,>
  where P: ?Sized, {
  /// The mapping to apply.
  map: F,
  /// The parser to map.
  parser: P,
}

impl<F, P,> Map<F, P,> {
  /// Constructs a new `Map` using `map` and `parser`.
  #[inline]
  pub const fn new(map: F, parser: P,) -> Self { Self { map, parser, } }
}

impl<F, P, I,> ParserFn<I,> for Map<F, P,>
  where F: Fn<(P::Output,)>,
    P: ParserFn<I,>, {
  type Output = F::Output;

  #[inline]
  fn parse(&self, input: I,) -> Parse<Self::Output, I,> {
    self.parser.parse(input,).map(&self.map,)
  }
}

impl<F, T, U,> CoerceUnsized<Map<F, U,>> for Map<F, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

fn _assert_coerce_unsized(a: Map<(), &i32,>,) {
  let _: Map<(), &dyn Send,> = a;
}