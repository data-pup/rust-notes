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

### WebAssembly Text Representation

Here is the `.wat` representation of the `unreachable_functions` module I wrote,
to test the `garbage` command:

```
(module
  (func $pushOne (result i32)
    i32.const 1
  )

  (func $unusedAddOne (param $val i32) (result i32)
    get_local $val
    call $pushOne
    i32.add)

  (func $addOne (param $val i32) (result i32)
    get_local $val
    call $pushOne
    i32.add)

  (func $double (param $val i32) (result i32)
    get_local $val
    get_local $val
    i32.add)

  (export "addOne" (func $addOne))
  (export "double" (func $double))
)
```

Note that `unusedAddOne` is not exported. `pushOne` is referenced by both
`addOne` and `unusedAddOne`, so it is transitively reachable, as `addOne` _is_
exported. `double` does not reference any other functions, and is exported
so it is not garbage either.

### Dominators

Running `twiggy dominators` on this module after compiling to wasm returns this
output:

```
 Retained Bytes │ Retained % │ Dominator Tree
────────────────┼────────────┼──────────────────────────
             28 ┊     35.00% ┊ export "addOne"
             19 ┊     23.75% ┊   ⤷ func[2]
             18 ┊     22.50% ┊       ⤷ code[2]
             10 ┊     12.50% ┊           ⤷ func[0]
              5 ┊      6.25% ┊               ⤷ code[0]
              4 ┊      5.00% ┊               ⤷ type[0]
             18 ┊     22.50% ┊ export "double"
              9 ┊     11.25% ┊   ⤷ func[3]
              8 ┊     10.00% ┊       ⤷ code[3]
              5 ┊      6.25% ┊ type[1]
`
```

The fact that the `func[1]` object does not show up anywhere in this tree is
a good clue that it is not reachable from the meta-root.

### Objdump

Next, I used the `wasm-objdump` tool found in the WABT toolkit to get some
more information about this, so I could correctly define my expected test
results.

Note that this binary expects a specific switch to be given.

__--details__

```
unreachable_functions.wasm:     file format wasm 0x1

Section Details:

Type:
 - type[0] () -> i32
 - type[1] (i32) -> i32
Function:
 - func[0] sig=0
 - func[1] sig=1
 - func[2] sig=1 <addOne>
 - func[3] sig=1 <double>
Export:
 - func[2] <addOne> -> "addOne"
 - func[3] <double> -> "double"
```

`func[0]` refers to `pushOne` here, and `func[1]` is our unreachable garbage
function. If the stack order did not give it away, you can also check this by
looking at the function signatures! Super neat :)

__--headers__

```
unreachable_functions.wasm:     file format wasm 0x1

Sections:

     Type start=0x0000000a end=0x00000014 (size=0x0000000a) count: 2
 Function start=0x00000016 end=0x0000001b (size=0x00000005) count: 4
   Export start=0x0000001d end=0x00000030 (size=0x00000013) count: 2
     Code start=0x00000032 end=0x00000050 (size=0x0000001e) count: 4
```

### Disaassembly

Next, I wanted to figure out the size of the unreachable functions, so I
disassembled the wasm binary and looked inside of it.

```
000033 func[0]:
 000035: 41 01                      | i32.const 1
 000037: 0b                         | end
000038 func[1]:
 00003a: 20 00                      | get_local 0
 00003c: 10 00                      | call 0
 00003e: 6a                         | i32.add
 00003f: 0b                         | end
000040 <addOne>:
 000042: 20 00                      | get_local 0
 000044: 10 00                      | call 0
 000046: 6a                         | i32.add
 000047: 0b                         | end
000048 <double>:
 00004a: 20 00                      | get_local 0
 00004c: 20 00                      | get_local 0
 00004e: 6a                         | i32.add
 00004f: 0b                         | end
```

It would also be useful to consider the output of `twiggy top` here.

```
 Shallow Bytes │ Shallow % │ Item
───────────────┼───────────┼────────────────
             9 ┊    11.25% ┊ export "addOne"
             9 ┊    11.25% ┊ export "double"
             8 ┊    10.00% ┊ code[1]
             8 ┊    10.00% ┊ code[2]
             8 ┊    10.00% ┊ code[3]
             5 ┊     6.25% ┊ type[1]
             5 ┊     6.25% ┊ code[0]
             4 ┊     5.00% ┊ type[0]
             1 ┊     1.25% ┊ func[0]
             1 ┊     1.25% ┊ func[1]
             1 ┊     1.25% ┊ func[2]
             1 ┊     1.25% ┊ func[3]
```

We found out above that `func[1]` is the unreachable function that we are
trying to identify here, so we need to calculate the size of that function.
The function starts at `000038` and ends at `000040`, so the difference is
`10 - 8` in hexadecimal (base-16), or `16 - 8` in conventional decimal (base-10).
Neat! So we have a size of 8 bytes.
