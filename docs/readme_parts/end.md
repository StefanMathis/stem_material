# Serialization and deserialization

The serde integration is gated behind the `serde` feature flag.

Most of the types (except errors) in this crate implement serialization and
deserialization. See the docstrings of the individual types for details.

# Documentation

The doc images are created by a second crate located within its repository 
(`docs/create_doc_images`) which uses this crate and the awesome
[plotters](https://crates.io/crates/plotters) crate.
The images shown in this documentation can be created with `cargo run` from
within `docs/create_doc_images`.