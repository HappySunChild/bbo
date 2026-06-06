use crate::buffer::{big, little, read_i8, read_u8, write_i8, write_u8};
use std::io::{Read, Result as IoResult, Write};

macro_rules! read_bytes_from {
	($target:ident, $count:expr) => {{
		let mut buf = [0; $count];
		$target.read_exact(&mut buf)?;
		buf
	}};
}
macro_rules! write_bytes_to {
	($target:ident, $method:expr, $count:expr, $($args:expr),+) => {{
		let mut buf = [0; $count];
		$method(&mut buf, $($args,)+);
		$target.write_all(&buf)
	}};
}

/// Extends [`Read`] with methods for reading numbers.
pub trait ReadBytesExt: Read {
	/// Reads a [`u8`] from the reader.
	#[inline]
	fn read_u8(&mut self) -> IoResult<u8> {
		Ok(read_u8(&read_bytes_from!(self, 1)))
	}

	/// Reads a little-endian [`u16`] from the reader.
	#[inline]
	fn read_u16_le(&mut self) -> IoResult<u16> {
		Ok(little::read_u16(&read_bytes_from!(self, 2)))
	}
	/// Reads a big-endian [`u16`] from the reader.
	#[inline]
	fn read_u16_be(&mut self) -> IoResult<u16> {
		Ok(big::read_u16(&read_bytes_from!(self, 2)))
	}

	/// Reads a little-endian [`u32`] from the reader.
	#[inline]
	fn read_u32_le(&mut self) -> IoResult<u32> {
		Ok(little::read_u32(&read_bytes_from!(self, 4)))
	}
	/// Reads a big-endian [`u32`] from the reader.
	#[inline]
	fn read_u32_be(&mut self) -> IoResult<u32> {
		Ok(big::read_u32(&read_bytes_from!(self, 4)))
	}

	/// Reads a little-endian [`u64`] from the reader.
	#[inline]
	fn read_u64_le(&mut self) -> IoResult<u64> {
		Ok(little::read_u64(&read_bytes_from!(self, 8)))
	}
	/// Reads a big-endian [`u64`] from the reader.
	#[inline]
	fn read_u64_be(&mut self) -> IoResult<u64> {
		Ok(big::read_u64(&read_bytes_from!(self, 8)))
	}

	/// Reads a little-endian [`u128`] from the reader.
	#[inline]
	fn read_u128_le(&mut self) -> IoResult<u128> {
		Ok(little::read_u128(&read_bytes_from!(self, 16)))
	}
	/// Reads a big-endian [`u128`] from the reader.
	#[inline]
	fn read_u128_be(&mut self) -> IoResult<u128> {
		Ok(big::read_u128(&read_bytes_from!(self, 16)))
	}

	/// Reads a little-endian, unsigned n-bytes integer from the reader as a [`u64`].
	#[inline]
	fn read_uint64_le(&mut self, byte_count: usize) -> IoResult<u64> {
		Ok(little::read_uint64(&read_bytes_from!(self, 8), byte_count))
	}
	/// Reads a big-endian, unsigned n-bytes integer from the reader as a [`u64`].
	#[inline]
	fn read_uint64_be(&mut self, byte_count: usize) -> IoResult<u64> {
		Ok(big::read_uint64(&read_bytes_from!(self, 8), byte_count))
	}

	/// Reads a little-endian, unsigned n-bytes integer from the reader as a [`u128`].
	#[inline]
	fn read_uint128_le(&mut self, byte_count: usize) -> IoResult<u128> {
		Ok(little::read_uint128(
			&read_bytes_from!(self, 16),
			byte_count,
		))
	}
	/// Reads a big-endian, unsigned n-bytes integer from the reader as a [`u128`].
	#[inline]
	fn read_uint128_be(&mut self, byte_count: usize) -> IoResult<u128> {
		Ok(big::read_uint128(&read_bytes_from!(self, 16), byte_count))
	}

	/// Reads a [`i8`] from the reader.
	#[inline]
	fn read_i8(&mut self) -> IoResult<i8> {
		Ok(read_i8(&read_bytes_from!(self, 1)))
	}

	/// Reads a little-endian [`i16`] from the reader.
	#[inline]
	fn read_i16_le(&mut self) -> IoResult<i16> {
		Ok(little::read_i16(&read_bytes_from!(self, 2)))
	}
	/// Reads a big-endian [`i16`] from the reader.
	#[inline]
	fn read_i16_be(&mut self) -> IoResult<i16> {
		Ok(big::read_i16(&read_bytes_from!(self, 2)))
	}

	/// Reads a little-endian [`i32`] from the reader.
	#[inline]
	fn read_i32_le(&mut self) -> IoResult<i32> {
		Ok(little::read_i32(&read_bytes_from!(self, 4)))
	}
	/// Reads a big-endian [`i32`] from the reader.
	#[inline]
	fn read_i32_be(&mut self) -> IoResult<i32> {
		Ok(big::read_i32(&read_bytes_from!(self, 4)))
	}

	/// Reads a little-endian [`i64`] from the reader.
	#[inline]
	fn read_i64_le(&mut self) -> IoResult<i64> {
		Ok(little::read_i64(&read_bytes_from!(self, 8)))
	}
	/// Reads a big-endian [`i64`] from the reader.
	#[inline]
	fn read_i64_be(&mut self) -> IoResult<i64> {
		Ok(big::read_i64(&read_bytes_from!(self, 8)))
	}

	/// Reads a little-endian [`i128`] from the reader.
	#[inline]
	fn read_i128_le(&mut self) -> IoResult<i128> {
		Ok(little::read_i128(&read_bytes_from!(self, 8)))
	}
	/// Reads a big-endian [`i128`] from the reader.
	#[inline]
	fn read_i128_be(&mut self) -> IoResult<i128> {
		Ok(big::read_i128(&read_bytes_from!(self, 8)))
	}

	/// Reads a little-endian, signed n-bytes integer from the reader as a [`u64`].
	#[inline]
	fn read_int64_le(&mut self, byte_count: usize) -> IoResult<i64> {
		Ok(little::read_int64(&read_bytes_from!(self, 8), byte_count))
	}
	/// Reads a big-endian, signed n-bytes integer from the reader as a [`u64`].
	#[inline]
	fn read_int64_be(&mut self, byte_count: usize) -> IoResult<i64> {
		Ok(big::read_int64(&read_bytes_from!(self, 8), byte_count))
	}

	/// Reads a little-endian, signed n-bytes integer from the reader as a [`u128`].
	#[inline]
	fn read_int128_le(&mut self, byte_count: usize) -> IoResult<i128> {
		Ok(little::read_int128(&read_bytes_from!(self, 16), byte_count))
	}
	/// Reads a big-endian, signed n-bytes integer from the reader as a [`u128`].
	#[inline]
	fn read_int128_be(&mut self, byte_count: usize) -> IoResult<i128> {
		Ok(big::read_int128(&read_bytes_from!(self, 16), byte_count))
	}

	/// Reads a little-endian [`f32`] from the reader.
	#[inline]
	fn read_f32_le(&mut self) -> IoResult<f32> {
		Ok(f32::from_bits(little::read_u32(&read_bytes_from!(self, 4))))
	}
	/// Reads a big-endian [`f32`] from the reader.
	#[inline]
	fn read_f32_be(&mut self) -> IoResult<f32> {
		Ok(f32::from_bits(big::read_u32(&read_bytes_from!(self, 4))))
	}

	/// Reads a little-endian [`f64`] from the reader.
	#[inline]
	fn read_f64_le(&mut self) -> IoResult<f64> {
		Ok(f64::from_bits(little::read_u64(&read_bytes_from!(self, 8))))
	}
	/// Reads a big-endian [`f64`] from the reader.
	#[inline]
	fn read_f64_be(&mut self) -> IoResult<f64> {
		Ok(f64::from_bits(big::read_u64(&read_bytes_from!(self, 8))))
	}
}
/// Extends [`Write`] with methods for writing numbers.
pub trait WriteBytesExt: Write {
	/// Write a [`u8`] to the writer.
	#[inline]
	fn write_u8(&mut self, n: u8) -> IoResult<()> {
		write_bytes_to!(self, write_u8, 1, n)
	}

	/// Writes a little-endian [`u16`] to the writer.
	#[inline]
	fn write_u16_le(&mut self, n: u16) -> IoResult<()> {
		write_bytes_to!(self, little::write_u16, 2, n)
	}
	/// Writes a big-endian [`u16`] to the writer.
	#[inline]
	fn write_u16_be(&mut self, n: u16) -> IoResult<()> {
		write_bytes_to!(self, big::write_u16, 2, n)
	}

	/// Writes a little-endian [`u32`] to the writer.
	#[inline]
	fn write_u32_le(&mut self, n: u32) -> IoResult<()> {
		write_bytes_to!(self, little::write_u32, 4, n)
	}
	/// Writes a big-endian [`u32`] to the writer.
	#[inline]
	fn write_u32_be(&mut self, n: u32) -> IoResult<()> {
		write_bytes_to!(self, big::write_u32, 4, n)
	}

	/// Writes a little-endian [`u64`] to the writer.
	#[inline]
	fn write_u64_le(&mut self, n: u64) -> IoResult<()> {
		write_bytes_to!(self, little::write_u64, 8, n)
	}
	/// Writes a big-endian [`u64`] to the writer.
	#[inline]
	fn write_u64_be(&mut self, n: u64) -> IoResult<()> {
		write_bytes_to!(self, big::write_u64, 8, n)
	}

	/// Writes a little-endian [`u128`] to the writer.
	#[inline]
	fn write_u128_le(&mut self, n: u128) -> IoResult<()> {
		write_bytes_to!(self, little::write_u128, 16, n)
	}
	/// Writes a big-endian [`u128`] to the writer.
	#[inline]
	fn write_u128_be(&mut self, n: u128) -> IoResult<()> {
		write_bytes_to!(self, big::write_u128, 16, n)
	}

	/// Writes a little-endian, unsigned n-byte integer as a [`u64`] to the writer.
	#[inline]
	fn write_uint64_le(&mut self, n: u64, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, little::write_uint64, 8, n, byte_count)
	}
	/// Writes a big-endian, unsigned n-byte integer as a [`u64`] to the writer.
	#[inline]
	fn write_uint64_be(&mut self, n: u64, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, big::write_uint64, 8, n, byte_count)
	}

	/// Writes a little-endian, unsigned n-byte integer as a [`u128`] to the writer.
	#[inline]
	fn write_uint128_le(&mut self, n: u128, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, little::write_uint128, 16, n, byte_count)
	}
	/// Writes a big-endian, unsigned n-byte integer as a [`u128`] to the writer.
	#[inline]
	fn write_uint128_be(&mut self, n: u128, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, big::write_uint128, 16, n, byte_count)
	}

	/// Write a [`i8`] to the writer.
	#[inline]
	fn write_i8(&mut self, n: i8) -> IoResult<()> {
		write_bytes_to!(self, write_i8, 1, n)
	}

	/// Writes a little-endian [`i16`] to the writer.
	#[inline]
	fn write_i16_le(&mut self, n: i16) -> IoResult<()> {
		write_bytes_to!(self, little::write_i16, 2, n)
	}
	/// Writes a big-endian [`i16`] to the writer.
	#[inline]
	fn write_i16_be(&mut self, n: i16) -> IoResult<()> {
		write_bytes_to!(self, big::write_i16, 2, n)
	}

	/// Writes a little-endian [`i32`] to the writer.
	#[inline]
	fn write_i32_le(&mut self, n: i32) -> IoResult<()> {
		write_bytes_to!(self, little::write_i32, 4, n)
	}
	/// Writes a big-endian [`i32`] to the writer.
	#[inline]
	fn write_i32_be(&mut self, n: i32) -> IoResult<()> {
		write_bytes_to!(self, big::write_i32, 4, n)
	}

	/// Writes a little-endian [`i64`] to the writer.
	#[inline]
	fn write_i64_le(&mut self, n: i64) -> IoResult<()> {
		write_bytes_to!(self, little::write_i64, 8, n)
	}
	/// Writes a big-endian [`i64`] to the writer.
	#[inline]
	fn write_i64_be(&mut self, n: i64) -> IoResult<()> {
		write_bytes_to!(self, big::write_i64, 8, n)
	}

	/// Writes a little-endian [`i128`] to the writer.
	#[inline]
	fn write_i128_le(&mut self, n: i128) -> IoResult<()> {
		write_bytes_to!(self, little::write_i128, 16, n)
	}
	/// Writes a big-endian [`i128`] to the writer.
	#[inline]
	fn write_i128_be(&mut self, n: i128) -> IoResult<()> {
		write_bytes_to!(self, big::write_i128, 16, n)
	}

	/// Writes a little-endian, signed n-byte integer as a [`i64`] to the writer.
	#[inline]
	fn write_int64_le(&mut self, n: i64, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, little::write_int64, 8, n, byte_count)
	}
	/// Writes a big-endian, signed n-byte integer as a [`i64`] to the writer.
	#[inline]
	fn write_int64_be(&mut self, n: i64, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, big::write_int64, 8, n, byte_count)
	}

	/// Writes a little-endian, signed n-byte integer as a [`i128`] to the writer.
	#[inline]
	fn write_int128_le(&mut self, n: i128, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, little::write_int128, 16, n, byte_count)
	}
	/// Writes a big-endian, signed n-byte integer as a [`i128`] to the writer.
	#[inline]
	fn write_int128_be(&mut self, n: i128, byte_count: usize) -> IoResult<()> {
		write_bytes_to!(self, big::write_int128, 16, n, byte_count)
	}

	/// Writes a little-endian [`f32`] to the writer.
	#[inline]
	fn write_f32_le(&mut self, n: f32) -> IoResult<()> {
		write_bytes_to!(self, little::write_f32, 4, n)
	}
	/// Writes a big-endian [`f32`] to the writer.
	#[inline]
	fn write_f32_be(&mut self, n: f32) -> IoResult<()> {
		write_bytes_to!(self, big::write_f32, 4, n)
	}

	/// Writes a little-endian [`f64`] to the writer.
	#[inline]
	fn write_f64_le(&mut self, n: f64) -> IoResult<()> {
		write_bytes_to!(self, little::write_f64, 8, n)
	}
	/// Writes a big-endian [`f64`] to the writer.
	#[inline]
	fn write_f64_be(&mut self, n: f64) -> IoResult<()> {
		write_bytes_to!(self, big::write_f64, 8, n)
	}
}

impl<R: Read> ReadBytesExt for R {}
impl<W: Write> WriteBytesExt for W {}
