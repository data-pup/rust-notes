```
This issue is reserved for someone who has not yet contributed to this codebase.

Currently we read the Cargo.toml twice! this is not preferred:

1.  read it to write package.json
2.  read it to pass crate name to wasm-bindgen

MENTORING INSTRUCTIONS (from @mgattozzi !):

1.  Create a small expression or function to read the Cargo.toml file to a String and assign it to a variable, maybe something like CARGO_TOML
2.  Replace instances of it being read with the variable you just created
3.  Remove any code associated with reading the file)
```

