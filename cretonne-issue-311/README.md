# Cretonne Issue 311 Work

Floating point instructions on x86, ARM, and other platforms produce NaN values
with differing bit patterns. We should add a mode that enables automatic
canonicalization of NaN values.

## Background

Floating point values are represented as a sign bit, an exponent, and a
mantissa. For a `f32` value, this looks like:

```
Sign (1 bit) Exponent (8 bits) Mantissa (23 bits)
0            11111111          00000000000000000000000
```

NaN is defined as a value where the exponent value is all 1's, and the mantissa
contains at least one non-zero bit. This means there are many ways to represent
NaN, which introduces nondeterminism.

The WebAssembly spec exposes this nondeterminism, rather than hiding it.

## Where NaN bits matter

Most of the time, the only thing that matters is that a value is _some_ kind
of NaN, not _which_ kind of NaN it is. However, they are observable in a few
specific types of circumstances:

*  When a NaN is stored to memory, and the bytes of memory are later inspected
*  When a floating-point value is bitcasted to integer
*  It is passed to a call or returned from a call to another function (which we assume might store the value)
*  It is the operand of a copysign
*  It is the operand of certain target-specific instructions

## Canonicalizing NaN's

For this case, canonicalizing a NaN means checking to see if a value is _some_
NaN, and replacing it with a specific 'default' NaN. This adds some overhead,
with the added benefit of determinism.

To test whether a value is NaN, we can use floating point equality comparisons.
Note that NaN is defined to be not equal to itself, while all other values
are equal to themselves. So, the easiest way to check if a value is NaN is:

```
let isNan = x != x;
```

A canonicalization sequence can consist of this comparison, followed by a
select, with an `f32const` or `f64const` to provide the canonical alternative.

## Implementing Notes

As a first step, we can insert canonicalizing code after every floating point
arithmetic instruction. This will add a fair amount of overhead, but will still
be faster than using floating-point libraries.

I should create a new pass, which can be enabled or disabled. This should visit
all instructions in a function, and replace each floating-point arithmetic
instruction with a sequence that performs the operation, followed by a
canonicalization step.

For these purposes, we can ignore `fneg`, `fabs`, and `fcopysign`, as these
only deal with the sign bit.

