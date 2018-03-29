# Rustc Intermediate Representation (IR) Notes

Intermediate Representation (IR) is the term for the data structures that the
compiler uses to represent source code internally while translating a program
into a binary that can be used by the computer.

In Rust, there are two levels of intermediate representation. The High-level
IR (HIR) is the primary IR used in `rustc`. The Mid-level IR (MIR) is the
second representation, which is a lower-level representation used by, among
other things, the borrow checker. We will discuss HIR first.

## High-level Intermediate Representation (HIR)

This representation is an abstract syntax tree (AST) created after the program
being compiled has had its macros expanded, names resolved, and had certain
high-level surface syntax constructs simplified.


```
bodies: {
        BodyId {
            node_id: NodeId(
                22
            )
        }: Body {
            arguments: [],
            value: expr(22: {
                ::io::_print(<::fmt::Arguments>::new_v1(&["Hello, world!\n"],
                                                        &match () { () => [], }));
            }),
            is_generator: false
        }
    },
    trait_impls: {},
    trait_auto_impl: {},
    body_ids: [
        BodyId {
            node_id: NodeId(
                22
            )
        }
    ]
}
```

