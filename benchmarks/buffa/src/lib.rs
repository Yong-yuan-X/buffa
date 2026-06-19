//! Generated protobuf types for buffa benchmarks (per-message isolated build).
//!
//! `build.rs` emits a `_include.rs` module tree (via buffa-build's
//! `include_file`) that wraps each generated file in a `pub mod` named after
//! its protobuf package. The iso message protos all declare `package bench`
//! and `benchmarks.proto` declares `package benchmarks`, so this yields the
//! `bench` and `benchmarks` modules the benches reference.
include!(concat!(env!("OUT_DIR"), "/_include.rs"));
