# Initial steps

The mentoring instructions given mention `aa3330d` as a good example of a
commit that demonstrates the steps to add a new subcommand. So, I will start
by looking through that commit, taking notes on the changes made, and then
moving on to take some more detailed notes regarding the crates in question.

## Adding a subcommand

That commit pertains to a specific merge commit, this is the section of the
log showing the commits made in that branch.

These commits come after `d958acf2a3a587dbfa1cff280b7669c78ae9e5be`.

```
*   ece63ac9d02cd1e12c131b8bbb07be4c2a8094ba Merge pull request #40 from rustwasm/monos
|\
| * aa3330d9981d6568550c86a55c50795fc954380c Implement an `twiggy monos` for finding monomorphization bloat
| * fe437dda2f6d884fe4dc6531b16dcf7bb6ebc2be Extract generic function names from monomorphizations
| * 5b23f531d0b33d4eaf913bd2201893ee18fb70e2 Fix indexing for function call edges and global reference edges
| * d65d68e4e893a05c5b1ef67aa3a6c88b23e18379 Add a fixture program containing a bunch of monomorphizations
```

Note: Commits below are analyzed in chronological order, i.e. going from the
bottom to the top in terms of order shown in the log.

### Commit d958acf2a3a587dbfa1cff280b7669c78ae9e5be

This adds a test fixture, which is compiled using the following command.

```
rustc +nightly -g --target wasm32-unknown-unknown monos.rs -o monos.wasm -C lto=fat -C opt-level=z
```

### 5b23f531d0b33d4eaf913bd2201893ee18fb70e2

This commit adds some extra logic in `parser/wasm.rs` to properly index
function call edges and global reference edges.

### fe437dda2f6d884fe4dc6531b16dcf7bb6ebc2be

This commit largely deals with the `Code` struct inside of `ir/ir.rs`.
The idea here being that it extracts generic function names from
monomorphizations. To quote the comments added here, "this is some hacky,
ad-hoc parsing shit!" So, I am not going to let myself get too bogged down
in the details of this commit, and accept that it is doing fancy stuff
regarding monomorphizations.

## Commit aa3330d9981d6568550c86a55c50795fc954380c

This is the commit that actually adds the `twiggy monos` subcommand itself,
after the previous commits added the groundwork. As such, this one will be
covered in slightly more detail.

### Monos

The following structs are added to `analyze/analyze.rs`, which represent
monomorphizations. `Monos` is really just a vector of entries. The entry
struct is a string (the name?), the id value(s) of the function, total size,
and the potential savings.

Note: We are not going to cover the monomorphization optimization in too much
detail, just to steps that went into adding this to `twiggy` to get a handle
on how we will add the `garbage` command.

```rust
#[derive(Debug)]
struct Monos {
    monos: Vec<MonosEntry>,
}

#[derive(Debug, PartialEq, Eq)]
struct MonosEntry {
    generic: String,
    insts: Vec<ir::Id>,
    total: u32,
    approx_potential_savings: u32,
}
```

The `Monos` struct implements the `Emit` trait. We will cover this later.

The other major changes are in `opt/definitions.rs` and `opt/opt.rs`.
`opt.rs` defines options for running `twiggy`. We will need to add a new
struct to the definitions file to define the options that the garbage
subcommand can run under. The changes in `opt.rs` are mostly just a matter
of adding an extra branch to match statements and so forth.

# The `analyze` crate

As most of this will be implemented in the `analyze` crate, I am going to
start by reviewing what is in this file.

This seems to be the table struct used to print information out.

```
#[derive(Debug, Clone)]
struct Table {
  header: Vec<(Align, String)>,
  rows: Vec<Vec<String>>,
}
```

# The `twiggy_ir` crate

The important objects that we will need to consider from the `twiggy_ir` crate
are `Item` and `Items`. An item is, surprise, an item in the binary.

```rust
/// An item in the binary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
  id: Id,
  name: String,
  size: u32,
  kind: ItemKind,
}
```

The items collection is defined like so:

```rust
#[derive(Debug)]
pub struct Items {
  size: u32,
  dominator_tree: Option<BTreeMap<Id, Vec<Id>>>,
  retained_sizes: Option<BTreeMap<Id, u32>>,
  predecessors: Option<BTreeMap<Id, Vec<Id>>>,
  items: Frozen<BTreeMap<Id, Item>>,
  edges: Frozen<BTreeMap<Id, Vec<Id>>>,
  roots: Frozen<BTreeSet<Id>>,
  meta_root: Id,
}
```

## Getting a list of items

We won't need to worry about the details of the IR crate to fix this issue,
the `run` function in `twiggy/twiggy.rs` creates a list of items using this
line.

```rust
let mut items = parser::read_and_parse(opts.input())?;
```

The line above creates an `ir:Items` collection.

Each of the subcommand functions in `analyze.rs` is defined with a signature
that looks something like this:

```rust
pub fn foo(items: &mut ir::Items, opts: &opt::Monos) -> Result<Box<traits::Emit>, traits::Error> {
}
```

# The external `petgraph` crate

The `petgraph` crate is used to perform a Dfs, so this is also worth
reviewing.

(Todo...)

## Dfs Traversal Example

This is an example of how to perform a depth-first search using the petgraph
visit implementation. This comes from the test bench. We will perform a similar
operation to traverse the IR when finding unreachable items.

```rust
let mut dfs = Dfs::new(&gr, a);
while let Some(next) = dfs.next(&gr) {
  println!("dfs visit => {:?}, weight={:?}", next, &gr[next]);
}
```

This struct is defined at `src/visit/traversal.rs` within the `petgraph` crate.

## Misc. Analysis Notes

A general pseudo-code layout of the function would look something like this:

```
let reachable_items = // Collect the reachable items using Dfs, shown above.

// Collect the items that are not reachable, collect them into a vector.
let mut garbabe_items: Vec<_> = items
  .iter()
  .filter(|item| // item.id() not in reachable items)
  .collect();

garbage_items.sort_by(
  // sort the items by size.
});

// Truncate the number of garbage items to be shown.
garbage_items.truncate(opts.number() as usize);

// Collect the id's of the garbage items.
let garbage_items: Vec<_> = garbage_items.into_iter().map(|i| i.id()).collect();

// Create the struct that we will return.
let garbo = Garbage {
  // ... (We will need to define a garbage struct.)
}

// Return the result in a boxed trait object. (?)
Ok(Box::new(garbo) as Box<traits::Emit>)
```

## Emit trait

One of the common option settings for all of the subcommands is the output.
The Emit trait is used to write objects to the given destination. Currently,
this only consists of text and Json, which must be implemented using the
following functions:

```
fn emit_text(&self, items: &ir::Items, destination: &mut io::Write) -> Result<(), Error>;
fn emit_json(&self, items: &ir::Items, destination: &mut io::Write) -> Result<(), Error>;
```

Note that these traits are implemented in the `analyze.rs` file.

Note: These trait implementations use the table struct (and its helpers) to
print a text representation.

