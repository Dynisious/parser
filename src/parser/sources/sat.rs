//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;
use core::ops::CoerceUnsized;

/// A parser which accepts tokens as long as they satisfy a predicate.
/// 
/// The predicate is passed the current token and the count of previously matched tokens.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Sat<F,>
  where F: ?Sized, {
  /// The predicate to apply.
  pub pred: F,
}

impl<F,> Sat<F,> {
  /// Constructs a new `Sat` with `pred`.
  #[inline]
  pub const fn new(pred: F,) -> Self { Sat { pred, } }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> F {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const F,) }
  }
}

impl<'a, F, I,> ParserFn<&'a [I],> for Sat<F,>
  where F: Fn<(usize, &'a I,), Output = bool> + ?Sized, {
  type Output = PResult<&'a [I], !,>;

  fn parse(&self, input: &'a [I],) -> Parse<Self::Output, &'a [I],> {
    let matched = input.iter().enumerate()
      .take_while(|&(i, t),| (self.pred)(i, t,),)
      .count();

    if matched < input.len() { Parse::from(input.split_at(matched,),).map(Output,) }
    else { Parse::new(Pending(1,), input,) }
  }
}

impl<F, U,> CoerceUnsized<Sat<U,>> for Sat<F,>
  where F: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A parser which accepts tokens as long as they satisfy a predicate requiring at least
/// one token to be accepted.
/// 
/// The predicate is passed the current token and the count of previously matched tokens.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Sat1<F,>
  where F: ?Sized, {
  /// The predicate to apply.
  pub pred: F,
}

impl<F,> Sat1<F,> {
  /// Constructs a new `Sat1` with `pred`.
  #[inline]
  pub const fn new(pred: F,) -> Self { Sat1 { pred, } }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> F {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const F,) }
  }
}

impl<'a, F, I,> ParserFn<&'a [I],> for Sat1<F,>
  where F: Fn<(usize, &'a I,), Output = bool> + ?Sized, {
  type Output = PResult<&'a [I], &'a [I; 1],>;

  fn parse(&self, input: &'a [I],) -> Parse<Self::Output, &'a [I],> {
    let matched = input.iter().enumerate()
      .take_while(|&(i, t),| (self.pred)(i, t,),)
      .count();

    if matched < input.len() {
      if matched == 0 { Parse::new(Failed(core::array::from_ref(&input[0],),), input,) }
      else { Parse::from(input.split_at(matched,),).map(Output,) }
    } else { Parse::new(Pending(1,), input,) }
  }
}

impl<F, U,> CoerceUnsized<Sat1<U,>> for Sat1<F,>
  where F: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

fn _assert_coerce_unsized(a: Sat<&i32,>, b: Sat1<&i32,>,) {
  let _: Sat<&dyn Send,> = a;
  let _: Sat1<&dyn Send,> = b;
}
