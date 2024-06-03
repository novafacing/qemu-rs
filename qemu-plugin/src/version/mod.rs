//! Tools for checking whether the plugin version and the
//! QEMU version are compatible

/// A mapping of the QEMU plugin version (given in sys::QEMU_PLUGIN_VERSION) to the last
/// QEMU version which supports that plugin version ( or "latest" if it is supported by
/// the latest version of QEMU)
pub const COMPABILITY_MAP: [(u8, &str); 3] = [(1, "8.2.3"), (2, "9.0.0"), (3, "latest")];
