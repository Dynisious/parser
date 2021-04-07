//! Transformers of the output type of a parser by sequencing multiple parsers.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::{*, combinators::*,};

/// A parser which maps a applies the output of one parser to the output of another.
pub type Apply<F, P,> = Pipe<SeqApply<P,>, F,>;
/// A parser which maps a applies the successful output of one parser to the successful
/// output of another.
pub type ApplyOk<F, P,> = Pipe<SeqApplyOk<P,>, F,>;
/// A parser which maps a applies the failed output of one parser to the failed output of
/// another.
pub type ApplyErr<F, P,> = Pipe<SeqApplyErr<P,>, F,>;
/// A parser which maps the output to another parser which is sequenced inline.
pub type Seq<F, P,> = Pipe<SeqPipe<F,>, P,>;
/// A parser which maps the successful output to another parser which is sequenced inline.
pub type SeqOk<F, P,> = Pipe<TrySeq<F,>, P,>;
/// A parser which maps the failure output to another parser which is sequenced inline.
pub type SeqErr<F, P,> = Pipe<TrySeqErr<F,>, P,>;

/// A parser which maps the parse to another parse.
#[derive(Clone, Copy, Default, Debug,)]
pub struct Pipe<F, P,> {
  /// The mapping to apply.
  map: F,
  /// The parser to map.
  parser: P,
}

impl<F, P,> Pipe<F, P,> {
  /// Constructs a new `Seq` using `map` and `parser`.
  #[inline]
  pub const fn new(map: F, parser: P,) -> Self { Pipe { map, parser, } }
}

impl<F, P, T, I,> FnOnce<(I,),> for Pipe<F, P,>
  where F: FnOnce(Parse<P::Value, I,>) -> Parse<T, I,>,
    P: ParserFnOnce<I,>, {
  type Output = Parse<T, I,>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (I,),) -> Self::Output { (self.map)((self.parser)(input,),) }
}

impl<F, P, T, I,> FnMut<(I,),> for Pipe<F, P,>
  where F: FnMut(Parse<P::Value, I,>) -> Parse<T, I,>,
    P: ParserFnMut<I,>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (I,),) -> Self::Output { (self.map)((self.parser)(input,),) }
}

impl<F, P, T, I,> Fn<(I,),> for Pipe<F, P,>
  where F: Fn(Parse<P::Value, I,>) -> Parse<T, I,>,
    P: ParserFn<I,>, {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (I,),) -> Self::Output { (self.map)((self.parser)(input,),) }
}
