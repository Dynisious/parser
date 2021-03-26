//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;
use core::ops::CoerceUnsized;

/// A parser which accepts the next token using a set of allowed tokens.
/// 
/// If an unexpected token occurs the it is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct OneOf<T,>
  where T: ?Sized, {
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

impl<'a, T, I,> ParserFn<&'a [I],> for OneOf<T,>
  where T: AsRef<[I]> + ?Sized,
    I: PartialEq, {
  type Output = PResult<&'a [I; 1], &'a [I; 1],>;

  fn call_parser(&self, input: &'a [I],) -> Parse<Self::Output, &'a [I],> {
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

impl<T, U,> CoerceUnsized<OneOf<U,>> for OneOf<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

/// A parser which accepts the next token using a set of forbidden tokens.
/// 
/// If an unexpected token occurs the it is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct NoneOf<T,>
  where T: ?Sized, {
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

impl<'a, T, I,> ParserFn<&'a [I],> for NoneOf<T,>
  where T: AsRef<[I]> + ?Sized,
    I: PartialEq, {
  type Output = PResult<&'a [I; 1], &'a [I; 1],>;

  fn call_parser(&self, input: &'a [I],) -> Parse<Self::Output, &'a [I],> {
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

impl<T, U,> CoerceUnsized<NoneOf<U,>> for NoneOf<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

fn _assert_coerce_unsized(a: OneOf<&i32,>, b: NoneOf<&i32,>,) {
  let _: OneOf<&dyn Send,> = a;
  let _: NoneOf<&dyn Send,> = b;
}
