//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::*;

/// A parser which accepts a specific sequence of tokens.
/// 
/// If an unexpected token occurs the matched prefix is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Tag<T,> {
  /// The expected sequence of tokens.
  pub tag: T,
}

impl<T,> Tag<T,> {
  /// Constructs a new `Tag` with `tag`.
  #[inline]
  pub const fn new(tag: T,) -> Self { Self { tag, } }
  /// Returns the inner value.
  #[inline]
  pub const fn into_inner(self,) -> T {
    use core::mem::MaybeUninit;

    unsafe { core::ptr::read(MaybeUninit::new(self,).as_ptr() as *const T,) }
  }
}

impl<'a, T, I,> FnOnce<(&'a [I],),> for Tag<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  type Output = Parse<PResult<&'a [I], &'a [I],>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (&'a [I],),) -> Self::Output { (&self)(input,) }
}

impl<'a, T, I,> FnMut<(&'a [I],),> for Tag<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output { (&*self)(input,) }
}

impl<'a, T, I,> Fn<(&'a [I],),> for Tag<T,>
  where T: AsRef<[I]>,
    I: PartialEq, {
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    let tag = self.tag.as_ref();
    let matched = tag.iter().zip(input,)
      .take_while(|(a, b,),| a == b,)
      .count();

    if matched == tag.len() { Parse::from(input.split_at(matched,),).map(Output,) }
    else { Parse::new(
      if matched == input.len() { Pending(tag.len() - matched,) }
      else { Failed(&input[..matched],) },
      input,
    ) }
  }
}
