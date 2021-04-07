//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

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

impl<T, I,> FnOnce<(I,),> for Always<T,>
  where T: Clone, {
  type Output = Parse<T, I,>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (I,),) -> Self::Output { Parse::new(self.value.clone(), input,) }
}

impl<T, I,> FnMut<(I,),> for Always<T,>
  where T: Clone, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (I,),) -> Self::Output { Parse::new(self.value.clone(), input,) }
}

impl<T, I,> Fn<(I,),> for Always<T,>
  where T: Clone, {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (I,),) -> Self::Output { Parse::new(self.value.clone(), input,) }
}
