# Overview Notes

The first thing I set about doing trying to understand more about how Servo
works is spending some time looking into how the project is organized, and
reading the various documentation files in the repository.

`docs/ORGANIZATION.md` contains some useful information about the structure
of the project, as well as a list of its major dependencies. This was a useful
place to start for me.

## Organization Notes

Most of Servo's code is in the `components/` directory. There are other
directories, such as `test/` and `etc/` that will be useful if you are
planning on working on Servo's code, but the bulk of the implementation details
will be found in `components/`.

### Traits Crates

Rust's traits system is one of my favorite parts of the language, and this
project uses this in a clever way. Some of the component crates are quite
complex, and build speeds would become especially slow for other crates if they
were to rely on these directly as a dependency.

To improve build times, the API's of these crates are exposed as a separate
`foo_traits` crate. This way, we can work with trait objects elsewhere without
needing to know about, or compile, the implementation details everytime that
we would like to interact with these components.

### Important Dependencies

There are a number of major dependencies listed in the organization document,
but these struck me as the most interesting.

github.com/servo/cssparser - A CSS Parser
github.com/hyperium/hyper - An HTTP implementation
github.com/servo/html5ever - An HTML5 Parser
github.com/servo/webrenderer - A GPU Renderer
github.com/cyderize/rust-websocket - A websocket protocol implementation

## Useful Terminology

Before we delve into any further details regarding specific components, it is
worth reading through the included glossary. You can find this at
`docs/glossary.md`. The list below does not cover all of the terminology,
just the terms that I found distinct and worth noting.

### Constellation

The thread that controls a collection of related web content. This could be
thought of as an owner of a single tab in a tabbed web browser; it encapsulates
session history, knows about all frames in a frame tree, and is the owner of
the pipeline for each contained frame.

### Display list,

A list of concrete rendering instructions. The display list is post-layout,
so all items have stacking-context-relative pixel positions, and z-index has
already been applied, so items later in the display list will always be on
top of items earlier in it.

### Pipeline

A unit encapsulating a means of communication with the script, layout, and
renderer threads for a particular document. Each pipeline has a globally-unique
id which can be used to access it from the constellation.

## Other Component Documentation

I was curious about finding other documentation regarding specific components.
Much of the documentation uses Markdown, so I searched the `components/`
directory for Markdown files using this command: `find components/ -type f -name "*.md" | less`

These all looked interesting to me, and are largely related to the most involved
systems in the components directory.

```
components/script/docs/JS-Servos-only-GC.md
components/hashglobe/README.md
components/style/README.md
components/selectors/README.md
components/profile/README.md
```

## Misc. Discoveries

This section contains miscellaneous details about Rust, Cargo that I learned
about while reading this code.

### Deny Unsafe Code

This disables unsafe code from being used in a file. Neat! For large projects
like Servo, it is useful to be able to explicitly deny specific features. This
saves the maintainers time, as they do not need to manually keep track of what
features are and are not being used as new changes are made to the code.

```rust
#![deny(unsafe_code)]
```

### Relative Dependencies

Dependencies within a project can be specified using relative paths! These
are contained within `{` and `}` braces.

```toml
[dependencies]
malloc_size_of = { path = "../malloc_size_of" }
malloc_size_of_derive = { path = "../malloc_size_of_derive" }
```

