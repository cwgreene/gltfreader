- Need to import Wrtie trait to be able to use it.
- I can't remember the boiler plate for clap.
- There's a convenience method for "read to end" which
  just takes a pathbuf
- Clap does support path names.
- There is shell support for clap, but it looks like
it's a separate project, and probably won't work out
of the box. Thus, if you distribute, you need to provide
instructions on how to take advantage of shell completion.

- Support for overriding single elements (to avoid printing out byte arrays)
is provided by the derivative crate.

https://crates.io/crates/derivative

Stackoverflow discussion here:
https://stackoverflow.com/questions/68151101/override-the-default-implementation-for-a-single-field-of-a-struct-using-deri

StackOverflow discussion about "{:?}" is here:

https://stackoverflow.com/questions/38157335/what-does-mean-in-a-rust-format-string
