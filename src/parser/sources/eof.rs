//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use crate::*;

/// A parser which expects an empty input.
/// 
/// If a non-empty input is given it is returned as the error.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
pub struct Eof;

impl<'a, I,> ParserFn<&'a [I],> for Eof {
  type Output = Result<&'a [I; 0], &'a [I]>;

  #[inline]
  fn parse(&self, input: &'a [I],) -> Parse<Self::Output, &'a [I],> {
    Parse::new(
      if input.is_empty() { Ok(&[]) }
      else { Err(input) },
      input,
    )
  }
}
