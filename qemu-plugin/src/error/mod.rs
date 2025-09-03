//! Errors that can occur in the qemu-plugin crate

#[derive(thiserror::Error, Debug)]
/// An error from the qemu-plugin crate
pub enum Error {
    #[error("Missing key for argument {argument}")]
    /// Error when an argument is missing a key
    MissingArgKey {
        /// The argument string a key is missing for
        argument: String,
    },
    #[error("Missing value for argument {argument}")]
    /// Error when an argument is missing a value
    MissingArgValue {
        /// The argument string a value is missing for
        argument: String,
    },
    #[error("Invalid boolean value {name} ({val})")]
    /// Error when a boolean argument is invalid
    InvalidBool {
        /// The name of the key-value argument pair which does not correctly parse as boolean
        name: String,
        /// The value of the key-value argument pair which does not correctly parse as boolean
        val: String,
    },
    #[error(
        "Setting the QEMU plugin uninstall callback was attempted concurrently and this attempt failed."
    )]
    /// Error when the QEMU plugin uninstall callback is set concurrently
    ConcurrentPluginUninstallCallbackSet,
    #[error(
        "Setting the QEMU plugin reset callback was attempted concurrently and this attempt failed."
    )]
    /// Error when the QEMU plugin reset callback is set concurrently
    ConcurrentPluginResetCallbackSet,
    #[error("Invalid state for plugin reset callback")]
    /// Error when the plugin reset callback is in an invalid state
    PluginResetCallbackState,
    #[error("Invalid instruction index {index} for translation block of size {size}")]
    /// Error when an instruction index is invalid
    InvalidInstructionIndex {
        /// The index into the translation block that is invalid
        index: usize,
        /// The size of the translation block
        size: usize,
    },
    #[error("No disassembly string available for instruction")]
    /// Error when no disassembly string is available for an instruction (i.e. NULL string
    NoDisassemblyString,
    #[error("Invalid size {size} for read of register {name}")]
    /// Error when the size of a register read is invalid
    InvalidRegisterReadSize {
        /// The register name
        name: String,
        /// The size of the attempted read
        size: usize,
    },
    #[error("Error while reading register {name}")]
    /// Error when reading a register fails
    RegisterReadError {
        /// The register name
        name: String,
    },
    #[error("Error while reading {len} bytes from virtual address {addr:#x}")]
    /// Error when reading memory from a virtual address fails
    VaddrReadError {
        /// The address read from
        addr: u64,
        /// The number of bytes read
        len: usize,
    },
    #[error(transparent)]
    /// A transparently wrapped `std::str::Utf8Error`
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    /// A transparently wrapped `std::ffi::NulError`
    NulError(#[from] std::ffi::NulError),
    #[error(transparent)]
    /// A transparently wrapped `anyhow::Error`
    Other(#[from] anyhow::Error),
}

/// Result type for the qemu-plugin crate
pub type Result<T> = std::result::Result<T, Error>;
