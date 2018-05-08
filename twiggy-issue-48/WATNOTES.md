# WAT Notes

## Overview

While working on this issue, I was having trouble compiling a wasm binary that
had unreachable items within it. As a result, I received some helpful advice
from @fitzgen that it would also be an option to write some wasm containing
unreachable code by hand.

This seemed like a fun opportunity to learn a little more about the WebAssembly
text format, and try out 'WABT' (pronounced 'wabbit'), the WebAssembly Binary
Toolkit.

You can find WABT on Github [here](https://github.com/WebAssembly/wabt).
Build instructions are included in the README.

## Fixture goals

Write a WASM module that includes some dead code, unreachable functions, etc.
Use WABT tools such as `wasm-objdump` to identify the size of items.

