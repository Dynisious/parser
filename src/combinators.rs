//! Combinator functions which are useful for building more complex parsers and some
//! common combinations of parsers.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::{*,
  parser::{mapping::{Map, MapOk, MapErr,},
  sequence::{Apply, ApplyOk, ApplyErr,},},
};
pub use combinators_rs::*;
use core::ops::Try;

/// Pairs the outputs of both parsers in a tuple.
pub type And<P, Q,> = Apply<Map<Pair, P,>, Q,>;
/// Pairs the successful outputs of both parsers in a tuple.
pub type AndOk<P, Q,> = ApplyOk<MapOk<Pair, P,>, Q,>;
/// Pairs the failure outputs of both parsers in a tuple.
pub type Or<P, Q,> = ApplyErr<MapErr<Pair, P,>, Q,>;

/// A function which applies the inner parser to the unused input before applying the
/// function parameter to the produced output.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct SeqApply<P,>(pub P,);

impl<P,> SeqApply<P,> {
  /// Constructs a `SeqApply` from `parser`.
  #[inline]
  pub const fn new(parser: P,) -> Self { SeqApply(parser,) }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> P {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const Self as *const P,) }
  }
  /// Maps the inner value.
  #[inline]
  pub fn map<Q, F,>(self, map: F,) -> SeqApply<Q,>
    where F: FnOnce(P,) -> Q, { SeqApply(map(self.0,),) }
}

impl<P, F, T, I,> FnOnce<(Parse<F, I,>,)> for SeqApply<P,>
  where P: ParserFnOnce<I,>,
    F: FnOnce(P::Value,) -> T, {
  type Output = Parse<T, I,>;

  #[inline]
  extern "rust-call" fn call_once(self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    self.0.parse_once(unused,).map(value,)
  }
}

impl<P, F, T, I,> FnMut<(Parse<F, I,>,)> for SeqApply<P,>
  where P: ParserFnMut<I,>,
    F: FnOnce(P::Value,) -> T, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    self.0.parse_mut(unused,).map(value,)
  }
}

impl<P, F, T, I,> Fn<(Parse<F, I,>,)> for SeqApply<P,>
  where P: ParserFn<I,>,
    F: FnOnce(P::Value,) -> T, {
  #[inline]
  extern "rust-call" fn call(&self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    self.0.parse(unused,).map(value,)
  }
}

/// A function which applies the inner parser to the unused input before applying the
/// function parameter to the successful produced output.
/// 
/// If either parse is an error the error is returned.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct SeqApplyOk<P,>(pub P,);

impl<P,> SeqApplyOk<P,> {
  /// Constructs a `SeqApplyOk` from `parser`.
  #[inline]
  pub const fn new(parser: P,) -> Self { SeqApplyOk(parser,) }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> P {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const Self as *const P,) }
  }
  /// Maps the inner value.
  #[inline]
  pub fn map<Q, F,>(self, map: F,) -> SeqApplyOk<Q,>
    where F: FnOnce(P,) -> Q, { SeqApplyOk(map(self.0,),) }
}

impl<P, F, G, T, U, E, I,> FnOnce<(Parse<F, I,>,)> for SeqApplyOk<P,>
  where P: ParserFnOnce<I,>,
    F: Try<Ok = G>,
    G: FnOnce(T,) -> U,
    E: From<F::Error>,
    P::Value: Try<Ok = T, Error = E>, {
  type Output = Parse<Result<U, E>, I,>;

  extern "rust-call" fn call_once(self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    let func = match value.into_result() {
      Ok(v) => v,
      Err(e) => return Parse::new(Err(e.into()), unused,),
    };
    match self.0.parse_once(unused,).into_result() {
      Ok(Parse { value, unused, }) => Parse::new(Ok(func(value,)), unused,),
      Err(parse,) => parse.map(|e,| Err(e.into()),),
    }
  }
}

impl<P, F, G, T, U, E, I,> FnMut<(Parse<F, I,>,)> for SeqApplyOk<P,>
  where P: ParserFnMut<I,>,
    F: Try<Ok = G>,
    G: FnMut(T,) -> U,
    E: From<F::Error>,
    P::Value: Try<Ok = T, Error = E>, {
  extern "rust-call" fn call_mut(&mut self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    let mut func = match value.into_result() {
      Ok(v) => v,
      Err(e) => return Parse::new(Err(e.into()), unused,),
    };
    match self.0.parse_mut(unused,).into_result() {
      Ok(Parse { value, unused, }) => Parse::new(Ok(func(value,)), unused,),
      Err(parse,) => parse.map(|e,| Err(e.into()),),
    }
  }
}

impl<P, F, G, T, U, E, I,> Fn<(Parse<F, I,>,)> for SeqApplyOk<P,>
  where P: ParserFn<I,>,
    F: Try<Ok = G>,
    G: Fn(T,) -> U,
    E: From<F::Error>,
    P::Value: Try<Ok = T, Error = E>, {
  extern "rust-call" fn call(&self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    let func = match value.into_result() {
      Ok(v) => v,
      Err(e) => return Parse::new(Err(e.into()), unused,),
    };
    match self.0.parse(unused,).into_result() {
      Ok(Parse { value, unused, }) => Parse::new(Ok(func(value,)), unused,),
      Err(parse,) => parse.map(|e,| Err(e.into()),),
    }
  }
}

/// A function which applies the inner parser to the unused input before applying the
/// function parameter to the failed produced output.
/// 
/// If either parse is a success the value is returned.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct SeqApplyErr<P,>(pub P,);

impl<P,> SeqApplyErr<P,> {
  /// Constructs a `SeqApplyErr` from `parser`.
  #[inline]
  pub const fn new(parser: P,) -> Self { SeqApplyErr(parser,) }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> P {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const Self as *const P,) }
  }
  /// Maps the inner value.
  #[inline]
  pub fn map<Q, F,>(self, map: F,) -> SeqApplyErr<Q,>
    where F: FnOnce(P,) -> Q, { SeqApplyErr(map(self.0,),) }
}

impl<P, F, G, T, E, U, I,> FnOnce<(Parse<F, I,>,)> for SeqApplyErr<P,>
  where P: ParserFnOnce<I,>,
    F: Try<Error = G>,
    G: FnOnce(E,) -> U,
    T: From<F::Ok>,
    P::Value: Try<Ok = T, Error = E>, {
  type Output = Parse<Result<T, U>, I,>;

  extern "rust-call" fn call_once(self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    let func = match value.into_result() {
      Ok(v) => return Parse::new(Ok(v.into()), unused,),
      Err(e) => e,
    };
    match self.0.parse_once(unused,).into_result() {
      Ok(parse,) => parse.map(|e,| Ok(e.into()),),
      Err(Parse { value, unused, }) => Parse::new(Err(func(value,)), unused,),
    }
  }
}

impl<P, F, G, T, E, U, I,> FnMut<(Parse<F, I,>,)> for SeqApplyErr<P,>
  where P: ParserFnMut<I,>,
    F: Try<Error = G>,
    G: FnMut(E,) -> U,
    T: From<F::Ok>,
    P::Value: Try<Ok = T, Error = E>, {
  extern "rust-call" fn call_mut(&mut self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    let mut func = match value.into_result() {
      Ok(v) => return Parse::new(Ok(v.into()), unused,),
      Err(e) => e,
    };
    match self.0.parse_mut(unused,).into_result() {
      Ok(parse,) => parse.map(|e,| Ok(e.into()),),
      Err(Parse { value, unused, }) => Parse::new(Err(func(value,)), unused,),
    }
  }
}

impl<P, F, G, T, E, U, I,> Fn<(Parse<F, I,>,)> for SeqApplyErr<P,>
  where P: ParserFn<I,>,
    F: Try<Error = G>,
    G: Fn(E,) -> U,
    T: From<F::Ok>,
    P::Value: Try<Ok = T, Error = E>, {
  extern "rust-call" fn call(&self, (Parse { value, unused, },): (Parse<F, I,>,),) -> Self::Output {
    let func = match value.into_result() {
      Ok(v) => return Parse::new(Ok(v.into()), unused,),
      Err(e) => e,
    };
    match self.0.parse(unused,).into_result() {
      Ok(parse,) => parse.map(|e,| Ok(e.into()),),
      Err(Parse { value, unused, }) => Parse::new(Err(func(value,)), unused,),
    }
  }
}

/// A function which applies the inner function to the parameter and applies
/// the returned parser to the unused input.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct SeqPipe<F,>(pub F,);

impl<F,> SeqPipe<F,> {
  /// Constructs a `SeqPipe` from `parser`.
  #[inline]
  pub const fn new(parser: F,) -> Self { SeqPipe(parser,) }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> F {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const Self as *const F,) }
  }
  /// Maps the inner value.
  #[inline]
  pub fn map<G, H,>(self, map: H,) -> SeqPipe<G,>
    where H: FnOnce(F,) -> G, { SeqPipe(map(self.0,),) }
}

impl<F, T, U, P, I,> FnOnce<(Parse<T, I,>,)> for SeqPipe<F,>
  where F: FnOnce(T,) -> P,
    P: ParserFnOnce<I, Value = U,>, {
  type Output = Parse<U, I,>;

  #[inline]
  extern "rust-call" fn call_once(self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    (self.0)(value,)(unused,)
  }
}

impl<F, T, U, P, I,> FnMut<(Parse<T, I,>,)> for SeqPipe<F,>
  where F: FnMut(T,) -> P,
    P: ParserFnOnce<I, Value = U,>, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    (self.0)(value,)(unused,)
  }
}

impl<F, T, U, P, I,> Fn<(Parse<T, I,>,)> for SeqPipe<F,>
  where F: Fn(T,) -> P,
    P: ParserFnOnce<I, Value = U,>, {
  #[inline]
  extern "rust-call" fn call(&self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    (self.0)(value,)(unused,)
  }
}

/// A function which applies the inner function to the successful parameter and applies
/// the returned parser to the unused input.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct TrySeq<F,>(pub F,);

impl<F,> TrySeq<F,> {
  /// Constructs a `TrySeq` from `parser`.
  #[inline]
  pub const fn new(parser: F,) -> Self { TrySeq(parser,) }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> F {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const Self as *const F,) }
  }
  /// Maps the inner value.
  #[inline]
  pub fn map<G, H,>(self, map: H,) -> TrySeq<G,>
    where H: FnOnce(F,) -> G, { TrySeq(map(self.0,),) }
}

impl<F, T, P, U, E, I,> FnOnce<(Parse<T, I,>,)> for TrySeq<F,>
  where F: FnOnce(T::Ok,) -> P,
    T: Try,
    P: ParserFnOnce<I, Value = U,>,
    U: Try<Error = E>,
    E: From<T::Error>, {
  type Output = Parse<Result<U::Ok, E>, I,>;

  extern "rust-call" fn call_once(self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    match value.into_result() {
      Ok(v) => (self.0)(v,)(unused,).map(U::into_result,),
      Err(e) => Parse::new(Err(e.into()), unused,),
    }
  }
}

impl<F, T, P, U, E, I,> FnMut<(Parse<T, I,>,)> for TrySeq<F,>
  where F: FnMut(T::Ok,) -> P,
    T: Try,
    P: ParserFnOnce<I, Value = U,>,
    U: Try<Error = E>,
    E: From<T::Error>, {
  extern "rust-call" fn call_mut(&mut self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    match value.into_result() {
      Ok(v) => (self.0)(v,)(unused,).map(U::into_result,),
      Err(e) => Parse::new(Err(e.into()), unused,),
    }
  }
}

impl<F, T, P, U, E, I,> Fn<(Parse<T, I,>,)> for TrySeq<F,>
  where F: Fn(T::Ok,) -> P,
    T: Try,
    P: ParserFnOnce<I, Value = U,>,
    U: Try<Error = E>,
    E: From<T::Error>, {
  extern "rust-call" fn call(&self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    match value.into_result() {
      Ok(v) => (self.0)(v,)(unused,).map(U::into_result,),
      Err(e) => Parse::new(Err(e.into()), unused,),
    }
  }
}

/// A function which applies the inner function to the failure parameter and applies the
/// returned parser to the unused input.
#[repr(transparent,)]
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct TrySeqErr<F,>(pub F,);

impl<F,> TrySeqErr<F,> {
  /// Constructs a `TrySeqErr` from `parser`.
  #[inline]
  pub const fn new(parser: F,) -> Self { TrySeqErr(parser,) }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> F {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const Self as *const F,) }
  }
  /// Maps the inner value.
  #[inline]
  pub fn map<G, H,>(self, map: H,) -> TrySeqErr<G,>
    where H: FnOnce(F,) -> G, { TrySeqErr(map(self.0,),) }
}

impl<F, T, P, U, I,> FnOnce<(Parse<T, I,>,)> for TrySeqErr<F,>
  where F: FnOnce(T::Error,) -> P,
    T: Try,
    P: ParserFnOnce<I, Value = U,>,
    U: Try,
    U::Ok: From<T::Ok>, {
  type Output = Parse<Result<U::Ok, U::Error>, I,>;

  extern "rust-call" fn call_once(self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    match value.into_result() {
      Ok(v) => Parse::new(Ok(v.into()), unused,),
      Err(e) => (self.0)(e,)(unused,).map(U::into_result,),
    }
  }
}

impl<F, T, P, U, I,> FnMut<(Parse<T, I,>,)> for TrySeqErr<F,>
  where F: FnMut(T::Error,) -> P,
    T: Try,
    P: ParserFnOnce<I, Value = U,>,
    U: Try,
    U::Ok: From<T::Ok>, {
  extern "rust-call" fn call_mut(&mut self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    match value.into_result() {
      Ok(v) => Parse::new(Ok(v.into()), unused,),
      Err(e) => (self.0)(e,)(unused,).map(U::into_result,),
    }
  }
}

impl<F, T, P, U, I,> Fn<(Parse<T, I,>,)> for TrySeqErr<F,>
  where F: Fn(T::Error,) -> P,
    T: Try,
    P: ParserFnOnce<I, Value = U,>,
    U: Try,
    U::Ok: From<T::Ok>, {
  extern "rust-call" fn call(&self, (Parse { value, unused, },): (Parse<T, I,>,),) -> Self::Output {
    match value.into_result() {
      Ok(v) => Parse::new(Ok(v.into()), unused,),
      Err(e) => (self.0)(e,)(unused,).map(U::into_result,),
    }
  }
}
