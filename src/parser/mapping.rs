//! Transformers of the output type of a parser by mapping the output value.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::{*, combinators::{TryMap, TryMapErr,},};

/// A parser which maps the successful output value of the inner parser.
pub type MapOk<F, P,> = Map<TryMap<F,>, P,>;
/// A parser which maps the failure output value of the inner parser.
pub type MapErr<F, P,> = Map<TryMapErr<F,>, P,>;

/// A parser which maps the output value of the inner parser.
#[derive(Clone, Copy, Default, Debug,)]
pub struct Map<F, P,> {
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

impl<F, P, T, I,> FnOnce<(I,),> for Map<F, P,>
  where F: FnOnce(P::Value,) -> T,
    P: ParserFnOnce<I,>, {
  type Output = Parse<T, I,>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (I,),) -> Self::Output {
    self.parser.parse_once(input,).map(self.map,)
  }
}

impl<F, P, T, I,> FnMut<(I,),> for Map<F, P,>
  where F: FnMut(P::Value,) -> T,
    P: ParserFnMut<I,>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (I,),) -> Self::Output {
    self.parser.parse_mut(input,).map(&mut self.map,)
  }
}

impl<F, P, T, I,> Fn<(I,),> for Map<F, P,>
  where F: Fn(P::Value,) -> T,
    P: ParserFn<I,>, {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (I,),) -> Self::Output {
    self.parser.parse(input,).map(&self.map,)
  }
}
