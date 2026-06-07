use crate::buffer::{big, little, read_i8, read_u8, write_i8, write_u8};

macro_rules! fn_runtime_read {
	($func:ident, $out:ty) => {
		#[inline]
		pub fn $func(self, buf: &[u8]) -> $out {
			match self {
				Endianness::Little => little::$func(buf),
				Endianness::Big => big::$func(buf),
			}
		}
	};
}
macro_rules! fn_runtime_read_count {
	($func:ident, $out:ty) => {
		#[inline]
		pub fn $func(self, buf: &[u8], byte_count: usize) -> $out {
			match self {
				Endianness::Little => little::$func(buf, byte_count),
				Endianness::Big => big::$func(buf, byte_count),
			}
		}
	};
}

macro_rules! fn_runtime_write {
	($func:ident, $out:ty) => {
		#[inline]
		pub fn $func(self, buf: &mut [u8], n: $out) {
			match self {
				Endianness::Little => little::$func(buf, n),
				Endianness::Big => big::$func(buf, n),
			}
		}
	};
}
macro_rules! fn_runtime_write_count {
	($func:ident, $out:ty) => {
		#[inline]
		pub fn $func(self, buf: &mut [u8], n: $out, byte_count: usize) {
			match self {
				Endianness::Little => little::$func(buf, n, byte_count),
				Endianness::Big => big::$func(buf, n, byte_count),
			}
		}
	};
}

/// Enum for describing what kind of byte order should be used when reading/writing numbers.
///
/// It is recommended that this is only used when the required type of endianness is not known at compile time.
#[derive(Clone, Copy)]
pub enum Endianness {
	/// Little endian
	Little,
	/// Big endian
	Big,
}

impl Endianness {
	#[inline]
	pub fn read_u8(self, buf: &[u8]) -> u8 {
		read_u8(buf)
	}
	fn_runtime_read!(read_u16, u16);
	fn_runtime_read!(read_u32, u32);
	fn_runtime_read!(read_u64, u64);
	fn_runtime_read!(read_u128, u128);
	fn_runtime_read_count!(read_uint64, u64);
	fn_runtime_read_count!(read_uint128, u128);

	#[inline]
	pub fn write_u8(self, buf: &mut [u8], n: u8) {
		write_u8(buf, n);
	}
	fn_runtime_write!(write_u16, u16);
	fn_runtime_write!(write_u32, u32);
	fn_runtime_write!(write_u64, u64);
	fn_runtime_write!(write_u128, u128);
	fn_runtime_write_count!(write_uint64, u64);
	fn_runtime_write_count!(write_uint128, u128);

	#[inline]
	pub fn read_i8(self, buf: &[u8]) -> i8 {
		read_i8(buf)
	}
	fn_runtime_read!(read_i16, i16);
	fn_runtime_read!(read_i32, i32);
	fn_runtime_read!(read_i64, i64);
	fn_runtime_read!(read_i128, i128);
	fn_runtime_read_count!(read_int64, i64);
	fn_runtime_read_count!(read_int128, i128);

	#[inline]
	pub fn write_i8(self, buf: &mut [u8], n: i8) {
		write_i8(buf, n);
	}
	fn_runtime_write!(write_i16, i16);
	fn_runtime_write!(write_i32, i32);
	fn_runtime_write!(write_i64, i64);
	fn_runtime_write!(write_i128, i128);
	fn_runtime_write_count!(write_int64, i64);
	fn_runtime_write_count!(write_int128, i128);

	fn_runtime_read!(read_f32, f32);
	fn_runtime_read!(read_f64, f64);

	fn_runtime_write!(write_f32, f32);
	fn_runtime_write!(write_f64, f64);
}

pub mod ext {
	use super::Endianness;
	use std::io::{Read, Result as IoResult, Write};

	macro_rules! read_bytes_from {
		($target:ident, $count:expr) => {{
			let mut buf = [0; $count];
			$target.read_exact(&mut buf)?;
			buf
		}};
	}
	macro_rules! write_bytes_to {
			($target:ident, $receiver:ident.$method:ident, $count:expr, $($args:expr),+) => {{
				let mut buf = [0; $count];
				$receiver.$method(&mut buf, $($args,)+);
				$target.write_all(&buf)
			}};
		}

	/// Extends [`Read`] with methods for reading numbers with a runtime-determined order.
	pub trait ReadBytesRuntimeExt: Read {
		/// Reads a [`u16`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_u16(&mut self, e: Endianness) -> IoResult<u16> {
			Ok(e.read_u16(&read_bytes_from!(self, 2)))
		}
		/// Reads a [`u32`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_u32(&mut self, e: Endianness) -> IoResult<u32> {
			Ok(e.read_u32(&read_bytes_from!(self, 4)))
		}
		/// Reads a [`u64`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_u64(&mut self, e: Endianness) -> IoResult<u64> {
			Ok(e.read_u64(&read_bytes_from!(self, 8)))
		}
		/// Reads a [`u128`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_u128(&mut self, e: Endianness) -> IoResult<u128> {
			Ok(e.read_u128(&read_bytes_from!(self, 16)))
		}
		/// Reads a unsigned n-bytes integer from the reader as a [`u64`] with the specified [`Endianness`].
		#[inline]
		fn read_uint64(&mut self, e: Endianness, byte_count: usize) -> IoResult<u64> {
			Ok(e.read_uint64(&read_bytes_from!(self, 8), byte_count))
		}
		/// Reads a unsigned n-bytes integer from the reader as a [`u128`] with the specified [`Endianness`].
		#[inline]
		fn read_uint128(&mut self, e: Endianness, byte_count: usize) -> IoResult<u128> {
			Ok(e.read_uint128(&read_bytes_from!(self, 16), byte_count))
		}

		/// Reads a [`i16`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_i16(&mut self, e: Endianness) -> IoResult<i16> {
			Ok(e.read_i16(&read_bytes_from!(self, 2)))
		}
		/// Reads a [`i32`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_i32(&mut self, e: Endianness) -> IoResult<i32> {
			Ok(e.read_i32(&read_bytes_from!(self, 4)))
		}
		/// Reads a [`i64`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_i64(&mut self, e: Endianness) -> IoResult<i64> {
			Ok(e.read_i64(&read_bytes_from!(self, 8)))
		}
		/// Reads a [`i128`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_i128(&mut self, e: Endianness) -> IoResult<i128> {
			Ok(e.read_i128(&read_bytes_from!(self, 16)))
		}
		/// Reads a signed n-bytes integer from the reader as a [`i64`] with the specified [`Endianness`].
		#[inline]
		fn read_int64(&mut self, e: Endianness, byte_count: usize) -> IoResult<i64> {
			Ok(e.read_int64(&read_bytes_from!(self, 8), byte_count))
		}
		/// Reads a signed n-bytes integer from the reader as a [`i128`] with the specified [`Endianness`].
		#[inline]
		fn read_int128(&mut self, e: Endianness, byte_count: usize) -> IoResult<i128> {
			Ok(e.read_int128(&read_bytes_from!(self, 16), byte_count))
		}

		/// Reads a [`f32`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_f32(&mut self, e: Endianness) -> IoResult<f32> {
			Ok(e.read_f32(&read_bytes_from!(self, 4)))
		}
		/// Reads a [`f64`] from the reader with the specified [`Endianness`].
		#[inline]
		fn read_f64(&mut self, e: Endianness) -> IoResult<f64> {
			Ok(e.read_f64(&read_bytes_from!(self, 8)))
		}
	}
	/// Extends [`Write`] with methods for writing numbers with a runtime-determined order.
	pub trait WriteBytesRuntimeExt: Write {
		/// Writes a [`u16`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_u16(&mut self, e: Endianness, n: u16) -> IoResult<()> {
			write_bytes_to!(self, e.write_u16, 2, n)
		}
		/// Writes a [`u32`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_u32(&mut self, e: Endianness, n: u32) -> IoResult<()> {
			write_bytes_to!(self, e.write_u32, 4, n)
		}
		/// Writes a [`u64`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_u64(&mut self, e: Endianness, n: u64) -> IoResult<()> {
			write_bytes_to!(self, e.write_u64, 8, n)
		}
		/// Writes a [`u128`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_u128(&mut self, e: Endianness, n: u128) -> IoResult<()> {
			write_bytes_to!(self, e.write_u128, 16, n)
		}
		/// Writes a unsigned n-bytes integer from the writer as a [`u64`] with the specified [`Endianness`].
		#[inline]
		fn write_uint64(&mut self, e: Endianness, n: u64, byte_count: usize) -> IoResult<()> {
			write_bytes_to!(self, e.write_uint64, 8, n, byte_count)
		}
		/// Writes a unsigned n-bytes integer from the writer as a [`u128`] with the specified [`Endianness`].
		#[inline]
		fn write_uint128(&mut self, e: Endianness, n: u128, byte_count: usize) -> IoResult<()> {
			write_bytes_to!(self, e.write_uint128, 16, n, byte_count)
		}

		/// Writes a [`i16`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_i16(&mut self, e: Endianness, n: i16) -> IoResult<()> {
			write_bytes_to!(self, e.write_i16, 2, n)
		}
		/// Writes a [`i32`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_i32(&mut self, e: Endianness, n: i32) -> IoResult<()> {
			write_bytes_to!(self, e.write_i32, 4, n)
		}
		/// Writes a [`i64`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_i64(&mut self, e: Endianness, n: i64) -> IoResult<()> {
			write_bytes_to!(self, e.write_i64, 8, n)
		}
		/// Writes a [`i128`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_i128(&mut self, e: Endianness, n: i128) -> IoResult<()> {
			write_bytes_to!(self, e.write_i128, 16, n)
		}
		/// Writes a unsigned n-bytes integer from the writer as a [`i64`] with the specified [`Endianness`].
		#[inline]
		fn write_int64(&mut self, e: Endianness, n: i64, byte_count: usize) -> IoResult<()> {
			write_bytes_to!(self, e.write_int64, 8, n, byte_count)
		}
		/// Writes a unsigned n-bytes integer from the writer as a [`i128`] with the specified [`Endianness`].
		#[inline]
		fn write_int128(&mut self, e: Endianness, n: i128, byte_count: usize) -> IoResult<()> {
			write_bytes_to!(self, e.write_int128, 8, n, byte_count)
		}

		/// Writes a [`f32`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_f32(&mut self, e: Endianness, n: f32) -> IoResult<()> {
			write_bytes_to!(self, e.write_f32, 4, n)
		}
		/// Writes a [`f64`] to the writer with the specified [`Endianness`].
		#[inline]
		fn write_f64(&mut self, e: Endianness, n: f64) -> IoResult<()> {
			write_bytes_to!(self, e.write_f64, 4, n)
		}
	}
}
