use std::io::{Read, Result as IOResult, Write};

macro_rules! read_bytes_method {
	($method:ident, $out:ty, $bytes:expr) => {
		#[inline]
		fn $method(&mut self) -> IOResult<$out> {
			let mut buf = [0; $bytes];
			self.read_exact(&mut buf)?;
			Ok(<$out>::from_le_bytes(buf))
		}
	};
}

macro_rules! write_bytes_method {
	($method:ident, $out:ty) => {
		#[inline]
		fn $method(&mut self, n: $out) -> IOResult<()> {
			self.write_all(&n.to_le_bytes())
		}
	};
}

use crate::buffer::{
	ReadBytes, ReadBytesFloat, ReadBytesSigned, WriteBytes, WriteBytesFloat, WriteBytesSigned,
};

impl<R: Read> ReadBytes for R {
	read_bytes_method!(read_u8, u8, 1);
	read_bytes_method!(read_u16_le, u16, 2);
	read_bytes_method!(read_u32_le, u32, 4);
	read_bytes_method!(read_u64_le, u64, 8);
	read_bytes_method!(read_u128_le, u128, 16);
}
impl<R: Read> ReadBytesSigned for R {}
impl<R: Read> ReadBytesFloat for R {}

impl<W: Write> WriteBytes for W {
	write_bytes_method!(write_u8, u8);
	write_bytes_method!(write_u16_le, u16);
	write_bytes_method!(write_u32_le, u32);
	write_bytes_method!(write_u64_le, u64);
	write_bytes_method!(write_u128_le, u128);
}
impl<W: Write> WriteBytesSigned for W {}
impl<W: Write> WriteBytesFloat for W {}
