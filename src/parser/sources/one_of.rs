//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::*;

/// A parser which accepts the next token using a set of allowed tokens.
/// 
/// If an unexpected token occurs the it is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct OneOf<T,> {
  /// The allowed tokens.
  pub one_of: T,
}

impl<T,> OneOf<T,> {
  /// Constructs a new `OneOf` with `one_of`.
  #[inline]
  pub const fn new(one_of: T,) -> Self { OneOf { one_of, } }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> T {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const T,) }
  }
}

impl<'a, T, I,> FnOnce<(&'a [I],),> for OneOf<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  type Output = Parse<PResult<&'a [I; 1], &'a [I; 1],>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (&'a [I],),) -> Self::Output { (&self)(input,) }
}

impl<'a, T, I,> FnMut<(&'a [I],),> for OneOf<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output { (&*self)(input,) }
}

impl<'a, T, I,> Fn<(&'a [I],),> for OneOf<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    match input.split_first() {
      None => Parse::new(Pending(1,), input,),
      Some((tok, unused,)) => {
        let tok = core::array::from_ref(tok,);

        if self.one_of.as_ref().contains(&tok[0],) {
          Parse::new(Output(tok,), unused,)
        } else { Parse::new(Failed(tok,), input,) }
      },
    }
  }
}

/// A parser which accepts the next token using a set of forbidden tokens.
/// 
/// If an unexpected token occurs the it is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct NoneOf<T,> {
  /// The forbidden tokens.
  pub none_of: T,
}

impl<T,> NoneOf<T,> {
  /// Constructs a new `NoneOf` with `none_of`.
  #[inline]
  pub const fn new(none_of: T,) -> Self { NoneOf { none_of, } }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> T {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const T,) }
  }
}

impl<'a, T, I,> FnOnce<(&'a [I],),> for NoneOf<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  type Output = Parse<PResult<&'a [I; 1], &'a [I; 1],>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (&'a [I],),) -> Self::Output { (&self)(input,) }
}

impl<'a, T, I,> FnMut<(&'a [I],),> for NoneOf<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output { (&*self)(input,) }
}

impl<'a, T, I,> Fn<(&'a [I],),> for NoneOf<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    match input.split_first() {
      None => Parse::new(Pending(1,), input,),
      Some((tok, unused,)) => {
        let tok = core::array::from_ref(tok,);

        if self.none_of.as_ref().contains(&tok[0],) {
          Parse::new(Failed(tok,), input,)
        } else { Parse::new(Output(tok,), unused,) }
      },
    }
  }
}
