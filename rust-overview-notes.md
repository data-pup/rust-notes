# Rust Notes

### Overview

While trying to build some things with Rust, I eventually hit the infamous
"fighting the borrow checker" phase of the learning curve. I should note that
this is a common problem, and not one that I was upset about! What is exciting
about Rust's syntactical ruleset is that it is helping you to write memory
safe code.

So, rather than getting frustrated by this, I decided to stop
and read some more documentation and see what helpful information I could find.
This repo contains some notes on Rust that helped me out while learning more
about the errors that were slowing me down while working.

Note: These notes presume that the reader is already slightly familiar with
Rust. You might be better off referring to the official documentation otherwise.

## Stack and Heap

Rust is a systems language. This means that Rust operates at a lower level than
other languages that you may be familiar with. This means that you may need to
learn some new concepts that were previously abstracted away from you. If you
are familiar with C or another C-like language, you may already understand how
to work with a stack and a heap, in which case this will be review.

In short, the stack is very fast, and is where memory is allocated by default.
The downside to the stack is that an allocation is local to a specific function
call, and is limited in size.

The heap, while slower, is effectively unlimited in size, and is globally
accessible.

### The Stack

```rust
fn main() {
    let x = 42;
}
```

When a function is called, memory is automatically allocated for all of its
local variables, and some other information. This is a 'stack frame'. When
`main` is called in the example above, a single 32-bit integer is allocated.
When `main` exits, this is automatically deallocated.

The amount of memory that should be allocated can be determined beforehand, and
can be added/removed very quickly using the stack. The problem with the stack is
that we will run into problems once we need to allocate variables that should
live longer than one function call.

### The Heap

Sometimes we need to have the ability to pass some information between different
functions, or keep it alive for longer than a single function's execution. When
these cases arise, we will use the heap.

In Rust, we use the `Box<T>` type to allocate memory on the heap. For example:

```rust
fn main() {
    let x = Box::new(5);
    let y = 42;
}
```

This would allocate a value on the heap, with the address of this value stored
in `x`.

## Iterators

Rather than using C-style for loops, Rust provides `for i in ...` loops, which
are a better alternative. Rather than C-style loops, we can iterate directly
using code that looks like this:

```rust
let nums = vec![1, 2, 3];

for num in &nums {
    println!("{}", num);
}
```

The benefit of using code like that above is that it is more direct about the
intention of what is being done, as well as being more efficient. Using a C-style
loop and indexing the vector leads to extra bounds checking. With the loop above,
we yield references to each elements, and avoid unnecessary bounds checks.

Note that `println!` can implicitly derefence the borrowed `&i32`, we could also
write `println!("{}", *num);`, but either way works.

## Consumers

A `consumer` operates on an iterator, returning some kind of value, or values.
`collect()` is an example of a consumer, which reads the values given by an
iterator, and returns a collection of the results.

`find(..)` is another example of a consumer. This function takes a closure as
a paramteres, and returns the first element that causes the closure to return
true. `find` returns an `Option`, which would be `None` if no elements
satisfied the conditions.

`fold(..)` is another consumer. Calls to fold look like
`fold(base, |acc, elem| ...)`, where base the initial accumulator state,
and the second parameter is a closure that processes the next element in a
collection using the current accumulator state.

## Iterator Adaptors

Iterator adaptors receive an iterator, modify it, and produce a new iterator.
The most common example of an iterator adaptor is `map`. Another example of
an iterator adaptor `take(n)` which will returns the next n elements given
by the original iterator.

`filter` is another example, which takes a closure that returns a bool, and
returns the elements given by the iterator that caused the closure to return
true.

## Concurrency

Concurrent code is difficult to code, and is an increasingly important topic.
Rust provides two traits to help make sense of code that can possibly be
concurrent.

The `Send` trait indicates that this type can have ownership transferred safely
between threads. The `Sync` trait indicates that this type has no possibility
of introducing memory unsafety when used from multiple threads concurrently
through shared references.

Before we continue further, we will look into how threads work in Rust.

## Threads

Rust's standard library provides a library for threads. This allows you to
run Rust code in parallel. Here is a simple example:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from a thread!"
    });

    println!("{}", handle.join().unwrap());
}
```

Note: Threads are a complex topic, and not what I had been having issues with,
so I am going to gloss over this for now.

### Note

This repo contains some more detailed notes on other topics as well.

### References:

Notes taken using the Rust documentation.

https://static.rust-lang.org/doc/master/book/
