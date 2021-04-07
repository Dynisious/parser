//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::*;

/// A parser which accepts tokens as long as they satisfy a predicate.
/// 
/// The predicate is passed the current token and the count of previously matched tokens.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Sat<F,> {
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

impl<'a, F, I,> FnOnce<(&'a [I],),> for Sat<F,>
  where F: FnMut(usize, &'a I,) -> bool, {
  type Output = Parse<PResult<&'a [I], !,>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(mut self, (input,): (&'a [I],),) -> Self::Output { (&mut self)(input,) }
}

impl<'a, F, I,> FnMut<(&'a [I],),> for Sat<F,>
  where F: FnMut(usize, &'a I,) -> bool, {
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output {
    let matched = input.iter().enumerate()
      .take_while(|&(i, t),| (self.pred)(i, t,),)
      .count();

    if matched < input.len() { Parse::from(input.split_at(matched,),).map(Output,) }
    else { Parse::new(Pending(1,), input,) }
  }
}

impl<'a, F, I,> Fn<(&'a [I],),> for Sat<F,>
  where F: Fn(usize, &'a I,) -> bool, {
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    let matched = input.iter().enumerate()
      .take_while(|&(i, t),| (self.pred)(i, t,),)
      .count();

    if matched < input.len() { Parse::from(input.split_at(matched,),).map(Output,) }
    else { Parse::new(Pending(1,), input,) }
  }
}

/// A parser which accepts tokens as long as they satisfy a predicate requiring at least
/// one token to be accepted.
/// 
/// The predicate is passed the current token and the count of previously matched tokens.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Sat1<F,> {
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

impl<'a, F, I,> FnOnce<(&'a [I],),> for Sat1<F,>
  where F: FnMut(usize, &'a I,) -> bool, {
  type Output = Parse<PResult<&'a [I], &'a [I; 1],>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(mut self, (input,): (&'a [I],),) -> Self::Output { (&mut self)(input,) }
}

impl<'a, F, I,> FnMut<(&'a [I],),> for Sat1<F,>
  where F: FnMut(usize, &'a I,) -> bool, {
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output {
    let matched = input.iter().enumerate()
      .take_while(|&(i, t),| (self.pred)(i, t,),)
      .count();

    if matched < input.len() {
      if matched == 0 { Parse::new(Failed(core::array::from_ref(&input[0],),), input,) }
      else { Parse::from(input.split_at(matched,),).map(Output,) }
    } else { Parse::new(Pending(1,), input,) }
  }
}

impl<'a, F, I,> Fn<(&'a [I],),> for Sat1<F,>
  where F: Fn(usize, &'a I,) -> bool, {
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    let matched = input.iter().enumerate()
      .take_while(|&(i, t),| (self.pred)(i, t,),)
      .count();

    if matched < input.len() {
      if matched == 0 { Parse::new(Failed(core::array::from_ref(&input[0],),), input,) }
      else { Parse::from(input.split_at(matched,),).map(Output,) }
    } else { Parse::new(Pending(1,), input,) }
  }
}
