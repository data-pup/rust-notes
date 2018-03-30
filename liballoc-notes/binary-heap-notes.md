# Binary Heap Standard Library Implementation Notes

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

Before we continue to the sifting helper functions, let's first review some
information coming from the information in `src/liballoc/btree/node.rs`. This
file remind us of an important point: Rust does not have dependent type, or
polymorphic recursion. As a result, structures like this involve writing
code that must be designated as unsafe.

I think the reasons for this are best understood by reading some code that
tangibly involves why these issues arise, so let's move back into the sifting
logic.

### Holes, Unsafety

The `sift_up` and `sift_down_to_bottom` methods make use of a struct called
`Hole`, found at `binary_heap.rs:865`, which we should consider first before
moving into the sift methods themselves.

```rust
struct Hole<'a, T: 'a> {
  data: &'a mut [T],
  elt: Option<T>,
  pos: usize,
}
```

When we are removing or adding an element, we will need to have the ability
to remove an element from the data vector, shift the others, and place it at
the final position in the `Hole.data` vector. By using a hole type, we can
perform operations with fewer moves, reducing the constant factor of the
runtime complexity.

### Sifting Up (Push)

To review, the `push` method looked like this:

```rust
pub fn push(&mut self, item: T) {
  let old_len = self.len();
  self.data.push(item);
  self.sift_up(0, old_len);
}

```

So we append an element to `data`, and then pass '0' with the previous data
length to the sifting function. That function should move the new value up to
its correct position in the vector. `sift_up` looks like this:

```rust
fn sift_up(&mut self, start: usize, pos: usize) -> usize {
  unsafe {
    let mut Hole = Hole::new(&mut self.data, pos);

    while hole.pos() > start {
      let parent (hole.pos() - 1) / 2;
      if hole.element() <= hole.get(parent) {
        break;
      }
      hole.move_to(parent);
    }
    hole.pos()
  }
}
```

The `Hole::new` method will remove the value at `pos`, creating a hole. The
idea here is that vector is searched from the end to the front, until the
correct position for the new element is found. At that point, the value at the
end is sifted up.

