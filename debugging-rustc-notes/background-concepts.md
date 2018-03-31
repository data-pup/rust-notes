# Background Concepts

Before we start attempting to debug `rustc`, we should first cover some
background concepts that will be important later.

### Bootstrapping

`rustc` is written in Rust, which means that it is a self-hosting compiler.
This term refers to the fact that the language that we are building a compiler
_for_ is also the language used to implement the compiler itself.

What this means for someone attempting to debug and patch a bug in `rustc` is
that we will need to use the latest nightly build of the compiler, and build
a new version using an edited copy of the source code stored in a local git
branch.

Once this version is built, we will then need to see if the problem still
occurs. This might also mean trying to implement a test case to check whether
the compiler panics on the given input.

