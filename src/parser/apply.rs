//! Applies the output of one parser to the output of another.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;
use core::ops::{CoerceUnsized, Try,};

/// A parser which applies the output from one parser to the output of another.
#[derive(Clone, Copy, Default, Debug,)]
pub struct Apply<F, P,>
  where P: ?Sized, {
  /// The function parser.
  func: F,
  /// The value parser.
  value: P,
}

impl<F, P,> Apply<F, P,> {
  /// Constructs a new `Apply` using `func` and `value`.
  #[inline]
  pub const fn new(func: F, value: P,) -> Self { Self { func, value, } }
}

impl<F, P, I,> ParserFn<I,> for Apply<F, P,>
  where F: ParserFn<I,>,
    P: ParserFn<I,>,
    F::Output: FnOnce<(P::Output,)>, {
  type Output = <F::Output as FnOnce<(P::Output,)>>::Output;

  #[inline]
  fn parse(&self, input: I,) -> Parse<Self::Output, I,> {
    let Parse { value, unused, } = self.func.parse(input,);
    self.value.parse(unused,).map(value,)
  }
}

impl<F, T, U,> CoerceUnsized<Apply<F, U,>> for Apply<F, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A parser which tries to apply the output from one parser to the output of another.
/// 
/// If either output errors the error is returned.
#[derive(Clone, Copy, Default, Debug,)]
pub struct TryApply<F, P,>
  where P: ?Sized, {
  /// The function parser.
  func: F,
  /// The value parser.
  value: P,
}

impl<F, P,> TryApply<F, P,> {
  /// Constructs a new `TryApply` using `func` and `value`.
  #[inline]
  pub const fn new(func: F, value: P,) -> Self { Self { func, value, } }
}

impl<F, P, I,> ParserFn<I,> for TryApply<F, P,>
  where F: ParserFn<I,>,
    P: ParserFn<I,>,
    F::Output: Try,
    P::Output: Try,
    <F::Output as Try>::Ok: FnOnce<(<P::Output as Try>::Ok,)>,
    <<F::Output as Try>::Ok as FnOnce<(<P::Output as Try>::Ok,)>>::Output: Try,
    <<<F::Output as Try>::Ok as FnOnce<(<P::Output as Try>::Ok,)>>::Output as Try>::Error: From<<F::Output as Try>::Error>,
    <<<F::Output as Try>::Ok as FnOnce<(<P::Output as Try>::Ok,)>>::Output as Try>::Error: From<<P::Output as Try>::Error>, {
  type Output = <<F::Output as Try>::Ok as FnOnce<(<P::Output as Try>::Ok,)>>::Output;

  fn parse(&self, input: I,) -> Parse<Self::Output, I,> {
    let Parse { value, unused, } = self.func.parse(input,)
      .into_result()
      //Handle any error.
      .map_err(|error,| error.map(From::from,),)?;
    match self.value.parse(unused,).into_result() {
      //Apply the function.
      Ok(parse) => parse.map(value,),
      //Handle any error.
      Err(error) => Parse::from_error(error.map(From::from,),)
    }
  }
}

impl<F, T, U,> CoerceUnsized<TryApply<F, U,>> for TryApply<F, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A parser which tries to apply the error from one parser to the error of another.
/// 
/// If either output succeeds the success is returned.
#[derive(Clone, Copy, Default, Debug,)]
pub struct TryApplyErr<F, P,>
  where P: ?Sized, {
  /// The function parser.
  func: F,
  /// The value parser.
  value: P,
}

impl<F, P,> TryApplyErr<F, P,> {
  /// Constructs a new `TryApplyErr` using `func` and `value`.
  #[inline]
  pub const fn new(func: F, value: P,) -> Self { Self { func, value, } }
}

impl<F, P, I,> ParserFn<I,> for TryApplyErr<F, P,>
  where F: ParserFn<I,>,
    P: ParserFn<I,>,
    F::Output: Try,
    P::Output: Try,
    <F::Output as Try>::Error: FnOnce<(<P::Output as Try>::Error,)>,
    <<F::Output as Try>::Error as FnOnce<(<P::Output as Try>::Error,)>>::Output: Try,
    <<<F::Output as Try>::Error as FnOnce<(<P::Output as Try>::Error,)>>::Output as Try>::Ok: From<<F::Output as Try>::Ok>,
    <<<F::Output as Try>::Error as FnOnce<(<P::Output as Try>::Error,)>>::Output as Try>::Ok: From<<P::Output as Try>::Ok>, {
  type Output = <<F::Output as Try>::Error as FnOnce<(<P::Output as Try>::Error,)>>::Output;

  fn parse(&self, input: I,) -> Parse<Self::Output, I,> {
    let Parse { value, unused, } = match self.func.parse(input,).into_result() {
      //Return the success.
      Ok(parse) => return Parse::from_ok(parse.map(From::from,),),
      Err(error) => error,
    };
    match self.value.parse(unused,).into_result() {
      //Return the success.
      Ok(parse) => Parse::from_ok(parse.map(From::from,),),
      //Apply the function.
      Err(error) => error.map(value,),
    }
  }
}

impl<F, T, U,> CoerceUnsized<TryApplyErr<F, U,>> for TryApplyErr<F, T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

fn _assert_coerce_unsized(a: Apply<(), &i32,>, b: TryApply<(), &i32,>, c: TryApplyErr<(), &i32,>,) {
  let _: Apply<(), &dyn Send,> = a;
  let _: TryApply<(), &dyn Send,> = b;
  let _: TryApplyErr<(), &dyn Send,> = c;
}