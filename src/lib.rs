//! cdetect can be used by build scripts to find and interrogate the C (and also C++) compilers
//! It also provides utilities for finding and interrogating other compilers (including RUSTC)

#![deny(warnings, missing_docs)]

/// The cdetect::properties module declares types with properties known about the C compiler and other compilers (and tools)
pub mod properties;
