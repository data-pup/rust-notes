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

