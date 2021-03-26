//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;

/// A parser which will always produce the same output.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Always<T,> {
  /// The output to produce.
  pub value: T,
}

impl<T,> Always<T,> {
  /// Constructs a new `Always` from `value`.
  #[inline]
  pub const fn new(value: T,) -> Self { Self { value, } }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> T {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const T,) }
  }
}

impl<T, I,> ParserFn<I,> for Always<T,>
  where T: Clone, {
  type Output = T;

  #[inline]
  fn call_parser(&self, input: I,) -> Parse<Self::Output, I,> {
    Parse::new(self.value.clone(), input,)
  }
}
