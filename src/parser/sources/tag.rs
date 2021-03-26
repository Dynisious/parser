//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;
use core::ops::CoerceUnsized;

/// A parser which accepts a specific sequence of tokens.
/// 
/// If an unexpected token occurs the matched prefix is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Tag<T,>
  where T: ?Sized, {
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

impl<'a, T, I,> ParserFn<&'a [I],> for Tag<T,>
  where T: AsRef<[I]> + ?Sized,
    I: PartialEq, {
  type Output = PResult<&'a [I], &'a [I],>;

  fn call_parser(&self, input: &'a [I],) -> Parse<Self::Output, &'a [I],> {
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

impl<T, U,> CoerceUnsized<Tag<U,>> for Tag<T,>
  where T: CoerceUnsized<U> + ?Sized,
    U: ?Sized, {}

fn _assert_coerce_unsized(a: Tag<&i32,>,) {
  let _: Tag<&dyn Send,> = a;
}
