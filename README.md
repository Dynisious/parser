
# Parser

A library of parser combinators.
 
There are two modes of parsing described:

* `lazy`: If the input buffer is exhausted before the parsing completes a `Pending`
result is returned indicating the minimum amount of data that should be appended to
the buffer before retrying. (the default)
* `strict`: All of the data necessary to complete the parse is expected to be in the
buffer before attempting. If the input buffer is exhausted before the parsing
completes a `Failed` result is returned.

## The [`Parser`] Type

The [`Parser`] type is the main entry-point into this libraries API.

The [`Parser`] type wraps other parsers and provides combinator methods for building more complex parsers.

There are implementations for the `Functor`, `Applicative`, `Monad`, `Alternative`, and
`Traversable` categories for [`Parser`] on both its `Output` and `Error` types.

## The [`curry`] Crate

The re-exported [`curry`] crate provides types for currying Rust functions such that you
can still write out the type signature, used and useful for the `Applicative` instance.

[`Parser`]: crate::Parser
[`curry`]: crate::curry
