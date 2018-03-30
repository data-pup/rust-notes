# Resource Acquisition is Initialization (RAII) Notes

Rust enforces a pattern called 'RAII', which helps prevent resource leak bugs.

This means that whenever an object goes out of scope, its destructor is called,
and all of its owned resources are freed. This removes the programmer's
responsibility to manually free allocated resources. This is possible through
Rust's notion of ownership. Variables, such as `Box<T>` own resources.

