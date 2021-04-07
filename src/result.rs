//! Defines the result types for parsers.
//! 
//! Author --- DMorgan  
//! Last Moddified --- 2021-03-31

use core::{
  fmt,
  ops::Try,
};

mod result;

pub use self::result::*;

/// The output of a parse.
#[derive(PartialEq, Eq, Clone, Copy,)]
pub struct Parse<T, Input,> {
  /// The produced value.
  pub value: T,
  /// The unused input.
  pub unused: Input,
}

impl<T, I,> Parse<T, I,> {
  /// Constructs a new `Parse` from `value` and `unused`.
  #[inline]
  pub const fn new(value: T, unused: I,) -> Self { Self { value, unused, } }
  /// Maps the inner `value` of this `Parse`.
  #[inline]
  pub fn map<U, F,>(self, f: F,) -> Parse<U, I,>
    where F: FnOnce(T,) -> U, { Parse::new(f(self.value,), self.unused,) }
}

impl<T, I,> Parse<Option<T>, I,> {
  /// Transposes the `Option` and `Parse`.
  #[inline]
  pub fn transpose(self,) -> Option<Parse<T, I,>> {
    let Parse { value, unused, } = self;
    Some(Parse::new(value?, unused,))
  }
}

impl<T, E, I,> Parse<Result<T, E>, I,> {
  /// Transposes the `Result` and `Parse`.
  #[inline]
  pub fn transpose(self,) -> Result<Parse<T, I,>, E> {
    let Parse { value, unused, } = self;
    Ok(Parse::new(value?, unused,))
  }
}

impl<T, I,> PartialEq<(T, I,),> for Parse<T, I,>
  where T: PartialEq,
    I: PartialEq, {
  #[inline]
  fn eq(&self, rhs: &(T, I,),) -> bool { self.value == rhs.0 && self.unused == rhs.1 }
}

impl<T, I,> Try for Parse<T, I,>
  where T: Try, {
  type Ok = Parse<T::Ok, I,>;
  type Error = Parse<T::Error, I,>;

  #[inline]
  fn into_result(self,) -> Result<Self::Ok, Self::Error> {
    match self.value.into_result() {
      Ok(v) => Ok(Parse::new(v, self.unused,)),
      Err(e) => Err(Parse::new(e, self.unused,)),
    }
  }
  #[inline]
  fn from_ok(ok: Self::Ok,) -> Self { ok.map(T::from_ok,) }
  #[inline]
  fn from_error(error: Self::Error,) -> Self { error.map(T::from_error,) }
}

impl<T, I,> From<(T, I,)> for Parse<T, I,> {
  #[inline]
  fn from((value, unused,): (T, I,),) -> Self { Self::new(value, unused,) }
}

impl<T, I,> fmt::Debug for Parse<T, I,>
  where T: fmt::Debug,
    I: fmt::Debug, {
  fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
    write!(fmt, "{:?}: >{:?}<", self.value, self.unused,)
  }
}

impl<T, I,> From<Parse<T, I,>> for (T, I,) {
  #[inline]
  fn from(Parse { value, unused, }: Parse<T, I,>,) -> (T, I,) { (value, unused,) }
}
