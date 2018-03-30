# Rustc Intermediate Representation (IR) Notes

Intermediate Representation (IR) is the term for the data structures that the
compiler uses to represent source code internally while translating a program
into a binary that can be used by the computer.

In Rust, there are two levels of intermediate representation. The High-level
IR (HIR) is the primary IR used in `rustc`. The Mid-level IR (MIR) is the
second representation, which is a lower-level representation used by, among
other things, the borrow checker. We will discuss HIR in this section.

## High-level Intermediate Representation (HIR)

This representation is an abstract syntax tree (AST) created after the program
being compiled has had its macros expanded, names resolved, and had certain
high-level surface syntax constructs simplified. Below is a shortened version
of the HIR tree representing the following source:

```
fn main() {
    println!("Hello, world!");
}
```

# HIR Tree:

```
Crate {
    module: Mod {
        inner: Span {
            // ...
        },
        item_ids: [
            // ...
    },
    attrs: [],
    span: Span {
        // ...
    },
    exported_macros: [],
    items: {
        // ...
    },
    trait_items: {},
    impl_items: {},
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

### Notes

Note that the top-level structure is the `Crate`. In our case, we have a single
module. The call to the `println!` macro can be seen in the `bodies` section.
This might seem like a complex object relative to the simplicity of the
program, but the `Crate` structure exposes some interfaces to allow for easier
access to different parts of the AST.

This structure also means that incremental compilation is easier. Becuase giving
access to a body only provides the corresponding node ID, the compiler can
record access dependencies.

This is only a cursory overview of HIR, but it is helpful to visualize what
kind of structure we will commonly be working with inside of the compiler code.

