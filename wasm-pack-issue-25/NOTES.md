# Initial Steps

The `Cargo.toml` file is read by the `read_cargo_toml` function in
`manifest.rs`, at line 45. This is invoked in two other places. This
information can all be figured out using the `ack` command, like so:

```
$: ack "read_cargo_toml"
src/manifest.rs
  45:fn read_cargo_toml(path: &str) -> Result<CargoManifest, Error> {
  96:    let crate_data = read_cargo_toml(path)?;
  116:    Ok(read_cargo_toml(path)?.package.name)
```

Next, let's look into the two invocations of the `read_cargo_toml` function.

1.  `get_crate_name`:
    *  This is a fairly simple function, read the `Cargo.toml` and return
       `CargoPackage.name` (a String). This function is used by the `init`
       command.
2.  `write_package_json`
    *  This function generates a `package.json` file in the `pkg` directory.
       This is a more complex function that does a number of things, but it
       does need to read the `Cargo.toml` as well.

### Applying `lazy_static`

This is the prototype for the `into_npm` method for the `CargoManifest` struct.

```
fn into_npm(mut self, scope: Option<String>) -> NpmPackage {
  // ...
}
```

This causes problems when the method is invoked on a static object, due to
a `cannot move out of borrowed content` error. This also happens when I
attempt to access the name field to get the crate name.

Thinking about how I can work around this.

On a different note, I can refactor the function used to read the `Cargo.toml`
file so that it will work with the std::env::args() object, and iterate
through each argument and check for a path. Use `skip(2)` to skip the initial
`wasm-pack` command and the subcommand. The first valid path found should be
used as the location for the `Cargo.toml` file.

Right now I am placing the static variable in the `manifest.rs` file, but this
may be moved later. Not sure yet, but this will either be placed here or in
`command.rs`.

## Update 05-04

I worked on this more this morning, and noticed some unexpected problems
that seem to pop up by using `lazy_static`.

Question: Should both of these commands work? If only one of these is required,
and the path is expected to always come either after, or before the options
for the subcommand, then we can proceed pretty easily.

```
wasm-pack init ./my/path --scope HEY   # Path first.
wasm-pack init --scope HEY ./my/path/  # Options first.
```

A concern I had about switching to the lazy static variable is that it in a
sense implicitly discards the `path` field of the command object, which could
lead to a confusing bug down the road.

This also makes testing some of the `manifest` functions a little difficult,
if we do not want to put unit tests directly into the files, relying instead
on integration tests.

One other route that we could consider is wrapping the commands into a struct
to represent the runtime context. This could lazily evaluate the `Cargo.toml`
contents as well, and avoid the issue of adding a noisy list of arguments to
the command functions.

