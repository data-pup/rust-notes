
Project initialization is performed in the main function under a branch that
matches `Command::Init`. Here is a high level overview of the function.

### Project Initialization

`Command::Init` is a struct containing two `Option<String>` variables, for path
and scope. Path is unwrapped, otherwise it is set to "."

1.  Add wasm target with rustup in `build::rustup_add_wasm_target`.
2.  Run `cargo build` in the given path targeting wasm, in `build::cargo_build_wasm`.
3.  Create a pkg directory using `wasm_pack::create_pkg_dir`.
4.  Create a `package.json` using the `manifest::write_package_json` function.
5.  Copy the crate's `README.md` into the package directory.
6.  Install `wasm-bindgen` using the `bindgen::cargo_install_wasm_bindgen()`.
7.  Get the name of the crate, using `manifest::get_crate_name(&crate_path)`.
8.  Run `wasm-bindgen`, using `wasm_bindgen_build(path: &str, name: &str)`.

### Crate Data

I am interested in seeing if I can refactory this so that the `Cargo.toml`
file is only read once. These are the places where crate data is read.

*  `manifest::write_package_json(&crate_path, scope)`
  *  Called from `main:52`
  *  Reads `Cargo.toml` and creates a `package.json` file, using
     the `CargoManifest::into_npm(&self, scope)` method.

*  `manifest::get_crate_name(&crate_path)`
  *  Called from `main:55`, used in `wasm-bindgen` build.

Writing `package.json`, and running `wasm-bindgen` are the two places where
some information must be read from the `Cargo.toml` file.

