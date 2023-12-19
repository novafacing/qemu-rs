#[derive(thiserror::Error, Debug)]
/// An error from the qemu-plugin crate
pub enum Error {
    #[error("Missing key for argument {argument}")]
    /// Error when an argument is missing a key
    MissingArgKey { argument: String },
    #[error("Missing value for argument {argument}")]
    /// Error when an argument is missing a value
    MissingArgValue { argument: String },
    #[error("Invalid boolean value {name} ({val})")]
    /// Error when a boolean argument is invalid
    InvalidBool { name: String, val: String },
    #[error("Setting the QEMU plugin uninstall callback was attempted concurrently and this attempt failed.")]
    /// Error when the QEMU plugin uninstall callback is set concurrently
    ConcurrentPluginUninstallCallbackSet,
    #[error("Setting the QEMU plugin reset callback was attempted concurrently and this attempt failed.")]
    /// Error when the QEMU plugin reset callback is set concurrently
    ConcurrentPluginResetCallbackSet,
    #[error("Invalid state for plugin reset callback")]
    /// Error when the plugin reset callback is in an invalid state
    PluginResetCallbackState,
    #[error("Invalid instruction index {index} for translation block of size {size}")]
    /// Error when an instruction index is invalid
    InvalidInstructionIndex { index: usize, size: usize },
    #[error("No disassembly string available for instruction")]
    /// Error when no disassembly string is available for an instruction (i.e. NULL string
    NoDisassemblyString,
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
