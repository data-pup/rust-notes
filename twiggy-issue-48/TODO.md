# TODO

## Adding test fixtures

*  Add a file, containing some garbage functions that are not referenced by
   any of the public functions etc. Note: I may need to add some attributes to
   disable compiler warnings regarding dead code.

*  Next, compile this targeting `wasm32-unknown-unknown`

*  Add the source and resulting `.wasm` binary to the project under the test
   fixtures directory.

## New struct

Add a new struct, to represent a collection of unreachable items, as well as
a single unreachable item. The struct will need to implement the `Emit` trait.

## Options

We will need to create a new item in the Options enum defined in `twiggy_opt`.

*  Add definition to `definitions.rs`
*  Add common cli options implementation to `opt.rs`

## Implementation

*  Traverse items starting from the meta-root using `petgraph::visit::Dfs` (reachable)
*  Traverse all items, filter for items that are not transitively reachable (garbage)
*  Print information about the unreachable items.


