# Adding a `twiggy garbage` subcommand to find unreachable items.

## Summary

Print code and data that is not transitively referenced by any exports or
public functions.

## To-Do

Print code and data

1.  Traverse `petgraph::visit::Dfs` starting from the meta-root node, to
    collect all items transitively reachable from some exported/public item.
2.  Iterate over all items, filtering for items that are -not- in the
    reachable set. These are the garbage items.
3.  Sort the garbage items by size.


Most of this will be implemented in `analyze/analyze.rs`, but will involve
changes to most of the crates.

`aa3330d` is a good example of adding a new sub-command.

