//! Author --- DMorgan  
//! Last Moddified --- 2021-03-26

use self::PResult::*;
use core::{
  fmt,
  ops::Try,
  iter::FromIterator,
  convert::TryFrom,
};

/// The result type of a lazy parser.
/// 
/// The `Pending` variant of this type allows parsers to indicate that they were not able
/// to determine success or failure and require more data before reattempting a parse.
#[must_use]
#[derive(PartialEq, Eq, Clone, Copy, Debug,)]
pub enum PResult<T, E,> {
  /// The value succeeded.
  Output(T,),
  /// The value did not have enough data to determine success or failure.
  Pending(usize,),
  /// The value failed.
  Failed(E,),
}

impl<T, E,> PResult<T, E,> {
  /// Contructs a new `Output` from `value`.
  #[inline]
  pub const fn new(value: T,) -> Self { Output(value,) }
  /// Maps the output type.
  #[inline]
  pub fn map<U, F,>(self, f: F,) -> PResult<U, E,>
    where F: FnOnce(T,) -> U, {
    match self {
      Output(value,) => Output(f(value,),),
      Pending(pending,) => Pending(pending,),
      Failed(error,) => Failed(error,),
    }
  }
  /// Maps the failed type.
  #[inline]
  pub fn map_fail<U, F,>(self, f: F,) -> PResult<T, U,>
    where F: FnOnce(E,) -> U, {
    match self {
      Output(value,) => Output(value,),
      Pending(pending,) => Pending(pending,),
      Failed(error,) => Failed(f(error,),),
    }
  }
  /// Maps the output type.
  #[inline]
  pub fn and_then<U, F,>(self, f: F,) -> PResult<U, E,>
    where F: FnOnce(T,) -> PResult<U, E>, {
    match self {
      Output(value,) => f(value,),
      Pending(pending,) => Pending(pending,),
      Failed(error,) => Failed(error,),
    }
  }
  /// Replaces an `Output` with `rhs`.
  #[inline]
  pub fn and<U,>(self, rhs: PResult<U, E,>,) -> PResult<U, E,> {
    self.and_then(#[inline] |_,| rhs,)
  }
  /// Maps the error type.
  #[inline]
  pub fn or_else<U, F,>(self, f: F,) -> PResult<T, U,>
    where F: FnOnce(E,) -> PResult<T, U>, {
    match self {
      Output(value,) => Output(value,),
      Pending(pending,) => Pending(pending,),
      Failed(error,) => f(error,),
    }
  }
  /// Replaces a non-`Output` with `rhs`.
  #[inline]
  pub fn or<U,>(self, rhs: PResult<T, U,>,) -> PResult<T, U,> {
    self.or_else(#[inline] |_,| rhs,)
  }
  /// Checks if `self` is an `Output` variant.
  #[inline]
  pub fn is_output(&self,) -> bool {
    match self {
      Output(_,) => true,
      _ => false,
    }
  }
  /// Checks if `self` is a `Pending` variant.
  #[inline]
  pub fn is_pending(&self,) -> bool {
    match self {
      Pending(_,) => true,
      _ => false,
    }
  }
  /// Checks if `self` is a `Failed` variant.
  #[inline]
  pub fn is_failed(&self,) -> bool {
    match self {
      Failed(_,) => true,
      _ => false,
    }
  }
  /// Gets the `Output` variant.
  #[inline]
  pub fn output(self,) -> Option<T> {
    match self {
      Output(value,) => Some(value),
      _ => None,
    }
  }
  /// Gets the `Pending` variant.
  #[inline]
  pub fn pending(self,) -> Option<usize> {
    match self {
      Pending(pending,) => Some(pending),
      _ => None,
    }
  }
  /// Gets the `Failed` variant.
  #[inline]
  pub fn failed(self,) -> Option<E> {
    match self {
      Failed(value,) => Some(value),
      _ => None,
    }
  }
  /// Unwraps an `Output` variant unchecked.
  /// 
  /// # Safety
  /// 
  /// If `self` is not an `Output` variant, behaviour is undefined.
  #[inline]
  pub unsafe fn unwrap_unchecked(self,) -> T {
    match self {
      Output(value,) => value,
      _ => {
        #[cfg(not(debug_assertions,),)]
        core::hint::unreachable_unchecked();
        #[cfg(debug_assertions,)]
        unreachable!("called `unwrap_unchecked` on a non `Output` variant.");
      },
    }
  }
  /// Unwraps a `Failed` variant unchecked.
  /// 
  /// # Safety
  /// 
  /// If `self` is not a `Failed` variant, behaviour is undefined.
  #[inline]
  pub unsafe fn unwrap_failed_unchecked(self,) -> E {
    match self {
      Failed(error,) => error,
      _ => {
        #[cfg(not(debug_assertions,),)]
        core::hint::unreachable_unchecked();
        #[cfg(debug_assertions,)]
        unreachable!("called `unwrap_failed_unchecked` on a non `Failed` variant.");
      },
    }
  }
}

impl<T, E,> PResult<T, E,>
  where E: fmt::Debug, {
  /// Unwraps an `Output` variant unchecked.
  /// 
  /// # Panics
  /// 
  /// If `self` is not an `Output` variant.
  #[inline]
  #[track_caller]
  pub fn unwrap(self,) -> T { self.expect("called `unwrap` on a non `Output` value") }
  /// Unwraps an `Output` variant unchecked.
  /// 
  /// # Panics
  /// 
  /// If `self` is not an `Output` variant.
  /// 
  /// # Params
  /// 
  /// msg --- The message to panic with.  
  #[inline]
  #[track_caller]
  pub fn expect(self, msg: &str,) -> T {
    match self {
      Output(value,) => value,
      Pending(pending,) => panic!("{}: {:?}", msg, Pending(pending,) as PResult<!, !,>,),
      Failed(error,) => panic!("{}: {:?}", msg, Failed(error,) as PResult<!, E,>,),
    }
  }
}

impl<T, E,> PResult<T, E,>
  where T: fmt::Debug, {
  /// Unwraps a `Failed` variant unchecked.
  /// 
  /// # Panics
  /// 
  /// If `self` is not a `Failed` variant.
  #[inline]
  #[track_caller]
  pub fn unwrap_failed(self,) -> E { self.expect_failed("called `unwrap_failed` on a non `Output` value") }
  /// Unwraps a `Failed` variant unchecked.
  /// 
  /// # Panics
  /// 
  /// If `self` is not a `Failed` variant.
  /// 
  /// # Params
  /// 
  /// msg --- The message to panic with.  
  #[inline]
  #[track_caller]
  pub fn expect_failed(self, msg: &str,) -> E {
    match self {
      Output(value,) => panic!("{}: {:?}", msg, Output(value,) as PResult<T, !,>,),
      Pending(pending,) => panic!("{}: {:?}", msg, Pending(pending,) as PResult<!, !,>,),
      Failed(error,) => error,
    }
  }
}

impl<T, E,> Try for PResult<T, E,> {
  type Ok = T;
  type Error = Result<E, usize>;

  #[inline]
  fn into_result(self,) -> Result<Self::Ok, Self::Error> {
    match self {
      Output(value,) => Ok(value),
      Pending(pending,) => Err(Err(pending)),
      Failed(error,) => Err(Ok(error)),
    }
  }
  #[inline]
  fn from_ok(value: Self::Ok,) -> Self { Output(value,) }
  #[inline]
  fn from_error(from: Self::Error,) -> Self {
    match from {
      Ok(error) => Failed(error,),
      Err(pending) => Pending(pending,),
    }
  }
}

impl<T, E,> From<Result<T, E>> for PResult<T, E,> {
  #[inline]
  fn from(from: Result<T, E>,) -> Self {
    match from {
      Ok(value) => Output(value,),
      Err(error) => Failed(error,),
    }
  }
}

impl<T, E,> From<Result<T, Result<E, usize>>> for PResult<T, E,> {
  #[inline]
  fn from(from: Result<T, Result<E, usize>>,) -> Self {
    from.map_or_else(Self::from_error, Output,)
  }
}

impl<T, E,> TryFrom<PResult<T, E>> for Result<T, E,> {
  type Error = usize;

  #[inline]
  fn try_from(from: PResult<T, E,>,) -> Result<Self, Self::Error> {
    match from {
      PResult::Output(value) => Ok(Ok(value,)),
      PResult::Pending(pending) => Err(pending),
      PResult::Failed(error) => Ok(Err(error,)),
    }
  }
}

impl<A, V, E,> FromIterator<PResult<A, E,>> for PResult<V, E,>
  where V: FromIterator<A>, {
  #[inline]
  fn from_iter<I,>(iter: I,) -> Self
    where I: IntoIterator<Item = PResult<A, E,>>, {
    iter.into_iter().map(PResult::into_result,).collect::<Result<V, _>>().into()
  }
}
