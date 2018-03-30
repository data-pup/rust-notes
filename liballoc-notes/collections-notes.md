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

There are some lines here that I would like to analyze further. Let's start
with this line, which is probably the busiest line in the function logically.

```rust
let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize:MAX).collect();
```

The `dish` vector is used to store the distances to each node starting from
the `start` node. Overall, this line initializes an element for each node
in the graph (represented by a `Vec<Edge>` in `adj_list`) to the maximum usize
value (using a closure given to `map(..)`), and collects them into a `Vec`
object using `collect`.

Next, let's look at the while loop below.

```rust
  while let Some(State { cost, position }) = heap.pop() {
    // ...
  }
```

This loop will attempt to destructure an `Option<State>` into the `const` and
`position` variables given by `heap.pop()`. When the pop function returns
`None`, this loop will terminate. Neat!

