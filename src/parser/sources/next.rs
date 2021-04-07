//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::*;
use core::convert::TryFrom;

/// A parser which accepts a given number of tokens from the front of the input.
#[derive(PartialEq, Eq, Clone, Copy, Default, Debug,)]
#[repr(transparent,)]
pub struct Next {
  /// The number of tokens to expect.
  pub count: usize,
}

impl Next {
  /// Constructs a new `Next` from `count`.
  #[inline]
  pub const fn new(count: usize,) -> Self { Next { count, } }
}

impl<'a, I,> FnOnce<(&'a [I],),> for Next {
  type Output = Parse<PResult<&'a [I], !,>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (&'a [I],),) -> Self::Output { (&self)(input,) }
}

impl<'a, I,> FnMut<(&'a [I],),> for Next {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output { (&*self)(input,) }
}

impl<'a, I,> Fn<(&'a [I],),> for Next {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    match self.count.checked_sub(input.len(),) {
      Some(pending) if pending > 0 => Parse::new(Pending(pending,), input,),
      _ => Parse::from(input.split_at(self.count,),).map(Output,),
    }
  }
}

impl<const N: usize,> From<NextN<N,>> for Next {
  #[inline]
  fn from(_: NextN<N,>,) -> Self { Next::new(N,) }
}

impl<const N: usize,> PartialEq<NextN<N,>> for Next {
  #[inline]
  fn eq(&self, _: &NextN<N,>,) -> bool { self.count == N }
}

/// A parser which accepts a given number of tokens from the front of the input.
#[derive(Eq, Clone, Copy, Default, Debug,)]
pub struct NextN<const COUNT: usize,>;

impl<const N: usize,> NextN<N,> {
  /// A `Next` with equivelant behaviour.
  pub const NEXT: Next = Next::new(N,);
}

impl<'a, I, const N: usize,> FnOnce<(&'a [I],),> for NextN<N,> {
  type Output = Parse<PResult<&'a [I; N], !,>, &'a [I],>;

  #[inline]
  extern "rust-call" fn call_once(self, (input,): (&'a [I],),) -> Self::Output { (&self)(input,) }
}

impl<'a, I, const N: usize,> FnMut<(&'a [I],),> for NextN<N,> {
  #[inline]
  extern "rust-call" fn call_mut(&mut self, (input,): (&'a [I],),) -> Self::Output { (&*self)(input,) }
}

impl<'a, I, const N: usize,> Fn<(&'a [I],),> for NextN<N,> {
  #[inline]
  extern "rust-call" fn call(&self, (input,): (&'a [I],),) -> Self::Output {
    match N.checked_sub(input.len(),) {
      Some(pending) if pending > 0 => Parse::new(Pending(pending,), input,),
      _ => {
        let (value, unused,) = input.split_at(N,);
        Parse::new(
          Output(unsafe {
            &*(value.as_ptr() as *const [I; N])
          },),
          unused,
        )
      },
    }
  }
}

impl<const N: usize,> TryFrom<Next> for NextN<N,> {
  type Error = Next;

  #[inline]
  fn try_from(from: Next,) -> Result<Self, Self::Error> {
    (from == Self).then_some(Self,).ok_or(from,)
  }
}

impl<const A: usize, const B: usize,> PartialEq<NextN<B,>> for NextN<A,> {
  #[inline]
  fn eq(&self, _: &NextN<B,>,) -> bool { A == B }
}

impl<const N: usize,> PartialEq<Next> for NextN<N,> {
  #[inline]
  fn eq(&self, rhs: &Next,) -> bool { N == rhs.count }
}
