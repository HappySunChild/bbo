/// Functions for reading/writing to buffers when the endianness is statically known.
pub mod buffer;
/// Extension traits for [`std::io::Write`] and [`std::io::Read`].
pub mod ext;
/// Utilities for reading/writing to buffers when the endianness is only known during runtime.
pub mod runtime;
