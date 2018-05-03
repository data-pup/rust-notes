# Writing a Pass

An example of how to write a pass can be found in `lib/codegen/src/preopt.rs`.

### Preopt Notes

Before implementing my own pass, I decided that I should take some notes on
the structure of `preopt.rs`, to gain a better understanding of what I will
need to do to accomplish this task.

```rust
/// The main pre-opt pass.
pub fn do_preopt(func: &mut Function) {
  let _tt = timing::preopt();
  let mut pos = FuncCursor::new(func);
  while let Some(_ebb) = pos.next_ebb() {
    while let Some(inst) = pos.next_inst() {
      // Apply basic simplifications.
      simplify(&mut pos, inst);

      //-- BEGIN -- division by constants ----------------

      let mb_dri = get_div_info(inst, &pos.func.dfg);
      if let Some(divrem_info) = mb_dri {
        do_divrem_transformation(&divrem_info, &mut pos, inst);
      continue;
      }

      //-- END -- division by constants ------------------
    }
  }
}
```

### Compilation Notes

The main compilation entrypoint is found in the `context` file. This file
declares and implements the `Context` structure. This is Cretonne's
compilation context, and main entry point.

The reasoning behind this is that we would like to avoid repeatedly allocating
and deallocating the data structures required for compilation, especially when
compiling a number of small functions.

This structure contains the function that is currently being compiled, along
with the function's control flow graph, dominator tree, register allocation
context, as well as loop analysis of the function.

The `compile` method contains the logic for compiling a function, and this is
where the passes defined in files like `preopt`, `postop`, and so forth are
actually invoked.

### Pass Steps

We will need to define an `f32const` and `f64const` with the canonical values
for NaN. After floating point arithmetic instructions, we should insert the
canonicalizing code that will check if a value is NaN, and replace it with
the corresponding canonical value. The `select` command will be useful here.

1.  Identifying instructions that are floating point arithmetic
2.  Code to canonicalize NaN
      *  Check if an instruction is a floating point arithmetic instruction.
      *  Check if value is NaN
      *  Replace with canonical NaN value if NaN, otherwise do not alter.
3.  Add pass to the compilation context.


### Instruction Information

Instructions are defined in `lib/codegen/meta/base/instructions.py`. This file
is used to automatically define the instructions. As mentioned before, `fneg`,
`fabs`, and `fcopysign` do not need to be considered by this pass. The
instructions to consider are:

`fadd, fsub, fmul, fdiv, sqrt, fma, fmin, fmax`

I am not sure about the rounding instructions, which include:

`ceil, floor, trunc, nearest`

My understanding is that these would -not- be considered, but I should confirm
this.

`Ieee32` and `Ieee64` are the representations of 32 bit and 64 bit floating
point values respectively. These are found in ir::immediates. Note that this
will be found in the `target` directory after building.

### Replacing result with alias to canonicalized result

dfg.rs contains the DataFlowGraph struct.

This contains a `replace_with_aliases` method, which should do what I need to
replace the nondeterminate value with a canonical version of the NaN result.

### Debugging Problems:

As of commit aa625633c935394ad1d065db73a20890936c4aef, I am getting closer to
a solution to this. While all of the codegen test suites pass, the following
error is thrown when I run the cretonne file-level tests.

NOTE: This is not yet adding the canonical NaN value, just the general control
flow that will be expected once I add a function to fetch/return a constant
value that will be used in place, assuming the result -is- a NaN value.

```
FAIL filetests/wasm/f32-arith.cton: compile(%f32_sqrt): inst3: arg 0 (v2) with type fflags failed to satisfy type set ValueTypeSet { lanes: BitSet(1), ints: BitSet(120), floats: BitSet(0), bools: BitSet(121) }
inst3: v3 = select.f32 v2, v1, v1

function %f32_sqrt(f32) -> f32 fast {
  ebb0(v0: f32):
  v1 = sqrt v0
  v2 = ffcmp v1, v1
  v3 = select v2, v1, v1
  return v1
}
```

After further work, I am getting closer. The return value is still not corect
in the function below, that will need some further work? However, the
canonicalizing code -seems- to be coming together.

```
function %f32_sqrt(f32) -> f32 fast {
  ebb0(v0: f32):
  v1 = sqrt v0
  v2 = ffcmp v1, v1
  v3 = f32const +sNaN:0x1
  v4 = select v2, v3, v1
  return v1
}
```

### Aliasing Result

The next step is wiring in the new result value. We do not want to rewire
all of the uses of the old value, because `fcmp` and `select` need to work
with this value still.

We can use `dfg`'s `replace_result` method the replace the result value of
the arithmetic instruction with a new `Value`. Then we can use `pos.ins()`'s
`with_result` when inserting the `select` using the original `Value`.

