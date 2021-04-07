//! Author --- DMorgan  
//! Last Moddified --- 2021-04-07

use crate::*;

/// A trait for once-off parsers.
/// 
/// A parser is a stateful computation which given some input produces a value and a new
/// input. In the case of most parsers the input will be some sequence of tokens where
/// some prefix is consumed to produce the output and the unused suffix is returned as
/// the new state.
pub trait ParserFnOnce<Input,>: FnOnce(Input,) -> Parse<Self::Value, Input,> + Sized {
  /// The output produced by the parser.
  type Value;

  /// Calls the parser.
  fn parse_once(self, input: Input,) -> Self::Output { self(input) }
}

impl<F, I, O,> ParserFnOnce<I,> for F
  where F: FnOnce(I,) -> Parse<O, I,>, {
  type Value = O;
}

/// A trait for repeatable parsers.
/// 
/// A parser is a stateful computation which given some input produces a value and a new
/// input. In the case of most parsers the input will be some sequence of tokens where
/// some prefix is consumed to produce the output and the unused suffix is returned as
/// the new state.
pub trait ParserFnMut<Input,>: ParserFnOnce<Input,> + FnMut(Input,) -> Parse<Self::Value, Input,> {
  /// Calls the parser.
  fn parse_mut(&mut self, input: Input,) -> Self::Output { self(input) }
}

impl<F, I, O,> ParserFnMut<I,> for F
  where F: FnMut(I,) -> Parse<O, I,>, {}

/// A trait for shared parsers.
/// 
/// A parser is a stateful computation which given some input produces a value and a new
/// input. In the case of most parsers the input will be some sequence of tokens where
/// some prefix is consumed to produce the output and the unused suffix is returned as
/// the new state.
pub trait ParserFn<Input,>: ParserFnMut<Input,> + Fn(Input,) -> Parse<Self::Value, Input,> {
  /// Call the parser.
  fn parse(&self, input: Input,) -> Self::Output { self(input) }
}

impl<F, I, O,> ParserFn<I,> for F
  where F: Fn(I,) -> Parse<O, I,>, {}
