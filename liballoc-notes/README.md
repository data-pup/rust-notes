# Liballoc Notes

While browsing through some of the rustc source, I noticed some lines that
imported various data structures from `std::collections`. Attempting to
implement some common data structures had proven to be deceptively difficult
for me while first learning Rust, so I thought that reading the official
implementations given in the standard library would be a valuable experience.

We will cover `liballoc`, but let's start by examining the code in the standard
collections library under `src/libstd/collections/`.

## The Rust Standard Collections Library

Rust provides implementations for common general purpose data structures. The
documentation does make an important disclaimer: In most cases, you are
probably better off using `Vec` or `HashMap`. That said, there are some rare
cases where another structure might be optimal, if performance is important.

The collections implemented can be loosely grouped into 4 different categories.
Each is also listed with a brief summary of when it should be used.
* Sequences
  *  Vec
    *  You want an ordered sequence of items, that you will usually append.
    *  You want a stack, resizable array, or a heap-allocated array
  *  VecDeque
    *  You want a Vec that efficiently insert at either end.
    *  You would like a double-ended queue (deque).
  *  LinkedList
    *  You would really, really like a linked list (These cases are rare).
* Maps
  *  HashMap
    *  You would like to associate keys with arbritrary values.
    *  You want a cache, or a map with no extra functionality.
  *  BTreeMap
    *  You want a map, sorted by its keys.
    *  You want to quickly obtain a range of the set.
    *  You want to quickly find the largest/smallest key-value pair.
* Sets
  *  HashSet, BTreeSet
    *  You only care about knowing which keys have been seen before.
    *  There is not a value that you are associating with your key.
* Misc
  *  BinaryHeap
    *  You would like a priority queue.
    *  You would like to store a large collection, but are only interested
       in processing the largest/smallest item at a time.

### Notes on Capacity

Most of these collections are implemented on top of an array, so it is worth
having an understanding of how to efficiently manage the capacity of a
collection.

`with_capacity` constructors will instruct the collection to make enough space
for the specified number of elements. For certain collections, this may not be
exactly accurate, but this should be used when you at least have an upper bound
for the number of elements that should be inserted into a collection.

The `reserve` method family is similar, and can be used to adjust capacity in
preparation for a batch of new elements. `shrink_to_fit` will shrink a
collection to the minimum size needed to hold its contents.

### Iterators

Iterators are an especially important fundamental for learning how to work with
collections in Rust. The benefit of iterators is that they provide a sequence
of values in a generic, safe, and efficient way.

The three most common iterators that almost every collection provides are
`iter`, `iter_mut`, and `into_iter`. `iter` provides immutable references to
the elements in the collection. `iter_mut` provides mutable references in the
same manner as `iter.`. Finally `into_iter` transforms the collection itself
into an iterator. This is useful for converting to/from different collections.

Here is an example of using the `into_iter` method along with the `collect`
method in order to convert a `Vec` into a `VecDeque`.

```rust
use std::collections::VecDeque;
let vec = vec![1, 2, 3];
let buf: VecDeque<_> = vec.into_inter().collect();
```

### Character Counting Example

The documentation also includes a nice example of using a `BTreeMap` to count
the occurences of a character in a string. This demonstrates some certain
patterns that are idiomatic to Rust, so I am including that here as well.

```rust
use std::collections::btree_map::BTreeMap;

let mut count = BTreeMap::new();
let message = "she sells sea shells by the sea shore";

for c in message.chars() {
  *count.entry(c).or_insert(0) += 1;
}

println!("Number of character occurences:");
for (char, count) in &count {
  println!("{}: {}", char, count);
}
```

This reads somewhat differently from other languages that you may be used to,
but it has the benefit of being concise, safe, and efficient.

### Finding Collection Implementations

So at this point, we might be interested in seeing how these structures are
implemented. Reading through the `use` statements in
`src/libstd/collections/mod.rs`, you will find that many these structures come
from the `alloc` crate, with the exception of `HashMap` and `HashSet`.

So, let's go explore what is in the `liballoc` folder, and see what we can
learn from the implementations of these structures.

## Rust Allocation Library

One important thing to note about the allocation library is that while it
contains the implementation of different structures, these are re-exported
from the collections library. So, the allocation library should almost never
be referred to directly by the programmer. Rather, this provides the internal
building blocks used by other crates in the standard library.

### Dijkstra's Algorithm Example

`src/liballoc/binary_heap.rs` contains a priority queue implemented using a
binary heap. This was a structure that I had an especially difficult time
attempting to implement myself, so this was a welcome discovery while perusing
the source. Even more useful, the documentation at the top of the file
contains an example implementation of Dijkstra's Algorithm. This algorithm
solves the shortest path problem for a directed graph.

...

