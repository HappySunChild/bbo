use std::io::Result as IOResult;

use crate::buffer::{
	ReadBytes, ReadBytesFloat, ReadBytesSigned, WriteBytes, WriteBytesFloat, WriteBytesSigned,
};

pub enum Endianness {
	Little,
	Big,
}

impl Endianness {
	#[inline]
	pub fn read_u8<R: ReadBytes>(self, buf: &mut R) -> IOResult<u8> {
		buf.read_u8()
	}
	#[inline]
	pub fn read_u16<R: ReadBytes>(self, buf: &mut R) -> IOResult<u16> {
		match self {
			Endianness::Little => buf.read_u16_le(),
			Endianness::Big => buf.read_u16_be(),
		}
	}
	#[inline]
	pub fn read_u32<R: ReadBytes>(self, buf: &mut R) -> IOResult<u32> {
		match self {
			Endianness::Little => buf.read_u32_le(),
			Endianness::Big => buf.read_u32_be(),
		}
	}
	#[inline]
	pub fn read_u64<R: ReadBytes>(self, buf: &mut R) -> IOResult<u64> {
		match self {
			Endianness::Little => buf.read_u64_le(),
			Endianness::Big => buf.read_u64_be(),
		}
	}
	#[inline]
	pub fn read_u128<R: ReadBytes>(self, buf: &mut R) -> IOResult<u128> {
		match self {
			Endianness::Little => buf.read_u128_le(),
			Endianness::Big => buf.read_u128_be(),
		}
	}

	#[inline]
	pub fn read_i8<R: ReadBytesSigned>(self, buf: &mut R) -> IOResult<i8> {
		buf.read_i8()
	}
	#[inline]
	pub fn read_i16<R: ReadBytesSigned>(self, buf: &mut R) -> IOResult<i16> {
		match self {
			Endianness::Little => buf.read_i16_le(),
			Endianness::Big => buf.read_i16_be(),
		}
	}
	#[inline]
	pub fn read_i32<R: ReadBytesSigned>(self, buf: &mut R) -> IOResult<i32> {
		match self {
			Endianness::Little => buf.read_i32_le(),
			Endianness::Big => buf.read_i32_be(),
		}
	}
	#[inline]
	pub fn read_i64<R: ReadBytesSigned>(self, buf: &mut R) -> IOResult<i64> {
		match self {
			Endianness::Little => buf.read_i64_le(),
			Endianness::Big => buf.read_i64_be(),
		}
	}
	#[inline]
	pub fn read_i128<R: ReadBytesSigned>(self, buf: &mut R) -> IOResult<i128> {
		match self {
			Endianness::Little => buf.read_i128_le(),
			Endianness::Big => buf.read_i128_be(),
		}
	}

	#[inline]
	pub fn read_f32<R: ReadBytesFloat>(self, buf: &mut R) -> IOResult<f32> {
		match self {
			Endianness::Little => buf.read_f32_le(),
			Endianness::Big => buf.read_f32_be(),
		}
	}
	#[inline]
	pub fn read_f64<R: ReadBytesFloat>(self, buf: &mut R) -> IOResult<f64> {
		match self {
			Endianness::Little => buf.read_f64_le(),
			Endianness::Big => buf.read_f64_be(),
		}
	}

	#[inline]
	pub fn write_u8<W: WriteBytes>(self, buf: &mut W, n: u8) -> IOResult<()> {
		buf.write_u8(n)
	}
	#[inline]
	pub fn write_u16<W: WriteBytes>(self, buf: &mut W, n: u16) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_u16_le(n),
			Endianness::Big => buf.write_u16_be(n),
		}
	}
	#[inline]
	pub fn write_u32<W: WriteBytes>(self, buf: &mut W, n: u32) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_u32_le(n),
			Endianness::Big => buf.write_u32_be(n),
		}
	}
	#[inline]
	pub fn write_u64<W: WriteBytes>(self, buf: &mut W, n: u64) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_u64_le(n),
			Endianness::Big => buf.write_u64_be(n),
		}
	}
	#[inline]
	pub fn write_u128<W: WriteBytes>(self, buf: &mut W, n: u128) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_u128_le(n),
			Endianness::Big => buf.write_u128_be(n),
		}
	}

	#[inline]
	pub fn write_i8<W: WriteBytesSigned>(self, buf: &mut W, n: i8) -> IOResult<()> {
		buf.write_i8(n as i8)
	}
	#[inline]
	pub fn write_i16<W: WriteBytesSigned>(self, buf: &mut W, n: i16) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_i16_le(n),
			Endianness::Big => buf.write_i16_be(n),
		}
	}
	#[inline]
	pub fn write_i32<W: WriteBytesSigned>(self, buf: &mut W, n: i32) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_i32_le(n),
			Endianness::Big => buf.write_i32_be(n),
		}
	}
	#[inline]
	pub fn write_i64<W: WriteBytesSigned>(self, buf: &mut W, n: i64) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_i64_le(n),
			Endianness::Big => buf.write_i64_be(n),
		}
	}
	#[inline]
	pub fn write_i128<W: WriteBytesSigned>(self, buf: &mut W, n: i128) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_i128_le(n),
			Endianness::Big => buf.write_i128_be(n),
		}
	}

	#[inline]
	pub fn write_f32<W: WriteBytesFloat>(self, buf: &mut W, n: f32) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_f32_le(n),
			Endianness::Big => buf.write_f32_be(n),
		}
	}
	#[inline]
	pub fn write_f64<W: WriteBytesFloat>(self, buf: &mut W, n: f64) -> IOResult<()> {
		match self {
			Endianness::Little => buf.write_f64_le(n),
			Endianness::Big => buf.write_f64_be(n),
		}
	}
}
