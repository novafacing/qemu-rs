//! Low level bindings to the QEMU Plugin API
//!
//! These bindings are generated from the QEMU source code, and should not be used directly.
//! Instead, use the `qeu-plugin` crate.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[cfg(feature = "plugin-api-v1")]
include!("bindings_v1.rs");

#[cfg(feature = "plugin-api-v2")]
include!("bindings_v2.rs");

#[cfg(feature = "plugin-api-v3")]
include!("bindings_v3.rs");
