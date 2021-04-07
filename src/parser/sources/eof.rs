//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::*;

/// A parser which expects an empty input.
/// 
/// If a non-empty input is given it is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Eof;

impl<'a, I,> FnOnce<(&'a [I],),> for Eof {
  type Output = Parse<Result<&'a [I; 0], &'a [I]>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (&'a [I],),) -> Self::Output {
    Parse::new(
      if input.is_empty() { Ok(&[]) }
      else { Err(input) },
      input,
    )
  }
}

impl<'a, I,> FnMut<(&'a [I],),> for Eof {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output {
    Parse::new(
      if input.is_empty() { Ok(&[]) }
      else { Err(input) },
      input,
    )
  }
}

impl<'a, I,> Fn<(&'a [I],),> for Eof {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    Parse::new(
      if input.is_empty() { Ok(&[]) }
      else { Err(input) },
      input,
    )
  }
}
