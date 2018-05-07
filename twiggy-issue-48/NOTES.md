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

Todo ...

### fe437dda2f6d884fe4dc6531b16dcf7bb6ebc2be

Todo ...

### aa3330d9981d6568550c86a55c50795fc954380c

Todo ...

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

To do...

