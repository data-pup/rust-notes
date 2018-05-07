# TODO

## Adding test fixtures

*  Add a file, containing some garbage functions that are not referenced by
   any of the public functions etc. Note: I may need to add some attributes to
   disable compiler warnings regarding dead code.

*  Next, compile this targeting `wasm32-unknown-unknown`

*  Add the source and resulting `.wasm` binary to the project under the test
   fixtures directory.

