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

First, we declare a `State` struct, and implement the `Ord` and `PartialOrd`
traits on this struct.

```rust
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  cost: usize,
  position: usize,
}

impl Ord for State {
  fn cmp(&self, other: &State) -> Ordering {
    other.cost.cmp(&self.cost)
      .then_with(|| self.position.cmp(&other.position))
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &State) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
```

Note that the Copy, Clone, Eq, and PartialEq traits can be derived
automatically, so that we do not need to manually implement simple boilerplate.
The ordering traits will allow us to compare the values between two states.

Next we implement an `Edge`, where each node is represented as a usize for
simplicity. Once this is defined, we can implement the pathfinding algorithm.

```rust
struct Edge {
  node: usize,
  cost: usize,
}
```

Using the structures we have now defined, this is how we can implement
Dijkstra's Algorithm:

```rust
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
  let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize:MAX).collect();

  let mut heap = BinaryHeap::new();
  dist[start] = 0;
  heap.push(State {cost: 0, position: start});

  while let Some(State { cost, position }) = heap.pop() {
    if position == goal { return Some(cost); }

    if cost > dist[position] { continue; }

    for edge in &adj_list[position] {
      let next = State { cost: cost + edge.cost, position: edge.node };

      if next.cost < dist [next.position] {
        heap.push(next);
        dist[next.position] = next.cost;
      }
    }
  }

  None
}
```

This example contains some extra explanatory comments in the documentation,
which I have stripped out here for the sake of brevity. Next, I would like
to cover some of the implementation details in the binary heap class, and
see what it can teach us about Rust collections.

## Binary Heap Standard Library Implementation

For this section, we will continue reading through the contents of
`src/liballoc/binary_heap.rs`, and progress to the heap implementation.
Let's start with the struct definition, which is relatively straightforward.

Note that this file is quite large, so we will only cover this class from
a high-level perspective. We will focus specifically on the `push` and `pop`
methods, and what makes them an interesting case study in Rust.

```rust
struct BinaryHeap<T> {
  data: Vec<T>,
}
```

As we read in the documentation earlier, many of these abstract structures
are built on top of a simple vector. The binary heap follows this pattern, and
only contains a vector internally. Simple enough, so let's next view the `impl`
blocks and see how this is used to expose a binary heap.

```rust
impl<T: Ord> BinaryHeap<T> {
  pub fn new() -> BinaryHeap<T> {
    BinaryHeap { data: vec![] }
  }


  pub fn peek(&self) -> Option<&T> {
    self.data.get(0)
  }

  pub fn push(&mut self, item: T) {
    let old_len = self.len();
    self.data.push(item);
    self.sift_up(0, old_len);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.data.pop().map(|mut item| {
      if !self.is_empty() {
        swap(&mut item, &mut self.data[0]);
        self.sift_down_to_bottom(0);
      }
      item
    })
  }
}
```

Notice that the `peek` method returns an `Option<&T>`. This makes sense, when
you consider that it should only return a value if the heap is not empty, and
the value returned should be an immutable reference if it does exist.

`push` and `pop` both involve an owned `T` type, which must be a type that
implements the `Ord` trait. These each make use of a private helper function
to sift a new value into its correct position, and to sort after the removal
of a new value, which is where some of the interesting consequences of Rust's
memory safety features occur.

### Sifting Up

Let's look at the code for `BinaryHeap::sift_up` next, to understand how the
heap is sorted after a new value has been appended to the data vector.

```rust
```

