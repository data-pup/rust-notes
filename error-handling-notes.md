# Error Handling

In general, error handling is divided into two broad categories:
exceptions and return values. Rust specifically opts to use return values
for error handling.

You can think of error handling as using case analysis, in order to determine
whether or not a computation was successful. The most straightforward way of
processing `Option<T>` types is using the `unwrap` method, but this will lead
to panic if the value is `None`.

Before we delve into this more, we should learn about the `Option` and `Result`
types. Both have an `unwrap` method defined.

### Options

The `Option` type is defined in the standard library, like this:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `Option` is used to express the possiblity of absence. This is important
for a number of reasons, and has a huge effect on the way that we write code
using Rust.

One common pattern is assigning a default value to the case that an Option is
is None. We can use `unwrap_or` for this, giving it a default value as a
parameter. `unwrap_or_else` is a more general version of this, which computes
the default value using an enclosure.

Another important combinator is `and_then`. This is helpful for composing
distinct computations that admit the possibility of absence.

### Results

The `Result` type is also defined in the standard library. It works like this:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The Result type is a richer version of Option, in that it expresses the
possibility of the absence of a value, but it also expresses the possiblity
of an error.

## Working with Options and Results Effectively

Let's look at a small example program that would panic in the event of
incorrect input. This program would parse an integer argument, double it,
and print the result.

```rust
use std::env;

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // error 1
    let n: i32 = arg.parse().unwrap(); // error 2
    println!("{}", 2 * n);
}
```

In order to compose the `Option` and `Result` types given by `nth()` and `parse`
respectively, we can use `ok_or` to convert the option into a `Result`. This
requires supplying the error to use if the result of `nth` is `None`. This can
be passed to `and_then`, which calls an operation if the result is `Ok`, and
returns the given `Err` value otherwise.

After making some changes to our program, we would have something like this:

```rust
use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

The main function calls `double_arg`, which will return the doubled value if
everything worked correctly, and an error containing a descriptive string if
a problem occured.

### In Defense of Unwrapping

While naively unwrapping is not ideal, it can be useful for a number of
reasons. During the initial draft of a program, it is useful to not need to
fully flesh out the error handling of the program, and focus on the logic
needed to accomplish the task at hand.

When better error handling is needed later, all of the points in your program
that should require thorough checks are clearly exposed. Consider using the
`Option::expect` method however, this can provide a better error message if a
panic occurs.

### The ? operator

This operator helps write compact code when dealing with error handling. The
purpose of this is that it will unpack the contents of a result if it is
`Ok`, and will return the `Err` otherwise.
