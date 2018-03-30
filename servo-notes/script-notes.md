# components/script Notes

The `script` directory contains the implementation of the Document Object
Model (DOM). This includes native Rust code, and bindings to Spider Monkey.
This sounded interesting to me, so I decided to dive into this and learn
more about how the DOM is built.

### Related Crates

There are 3 main crates related to `script`, which are described in the
organization document as follows:

* script
  * Implementation of the DOM (native Rust code and bindings to SpiderMonkey).
* script_layout_interface
  * The API the script crate provides for the layout crate.
* script_traits
  * APIs to the script crate for crates that don't want to depend on the
    script crate for build speed reasons.

Notably, the `layout` component interacts with the `script` component enough
that there is a separate directory besides `script_traits`. This relationship
will be investigated further, but let's first start by reading the `script`
documentation file found at `components/script/docs/JS-Servos-only-GC.md`, and
then examining the traits that are exposed in `script_traits`.

## "JavaScript: Servo's only garbage collector"

This document lays out some of the important conceptual points regarding the
memory management strategies used in Servo's DOM implementation. This section
of the writeup will only summarize key conceptual points that helped me, the
full file is worth reading if you are interested in learning more about this.

Layout and rendering algorithms used in browsers are often implemented in
native code, for performance reasons. JS code can alter the document however,
which means that the browser's representation of the document is cross-language
data structure.




