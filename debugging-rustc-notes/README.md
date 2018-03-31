# rustc Debugging Notes

## Problem Overview

### Problem Description

...

### Source Code



### Error Message


```
error[E0433]: failed to resolve. Use of undeclared type or module `std`
 --> src/args.rs:6:5
   |
   6 |     std::env::args()
     |     ^^^ Use of undeclared type or module `std`

     error: internal compiler error: librustc/ich/impls_ty.rs:906: ty::TypeVariants::hash_stable() - Unexpected variant TyInfer(?0).

     thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
     note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

