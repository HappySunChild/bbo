use std::io::{Read, Result, Write};

macro_rules! read_bytes {
	($from:expr, $bytes:expr) => {{
		let mut buf = [0; $bytes];
		$from.read_exact(&mut buf)?;
		buf
	}};
}

pub trait ReadUIntLiteral: Read {
	#[inline]
	fn read_u8(&mut self) -> Result<u8> {
		Ok(read_bytes!(self, 1)[0])
	}

	#[inline]
	fn read_u16_le(&mut self) -> Result<u16> {
		Ok(u16::from_le_bytes(read_bytes!(self, 2)))
	}
	#[inline]
	fn read_u16_be(&mut self) -> Result<u16> {
		Ok(u16::from_be_bytes(read_bytes!(self, 2)))
	}

	#[inline]
	fn read_u32_le(&mut self) -> Result<u32> {
		Ok(u32::from_le_bytes(read_bytes!(self, 4)))
	}
	#[inline]
	fn read_u32_be(&mut self) -> Result<u32> {
		Ok(u32::from_be_bytes(read_bytes!(self, 4)))
	}

	#[inline]
	fn read_u64_le(&mut self) -> Result<u64> {
		Ok(u64::from_le_bytes(read_bytes!(self, 8)))
	}
	#[inline]
	fn read_u64_be(&mut self) -> Result<u64> {
		Ok(u64::from_be_bytes(read_bytes!(self, 8)))
	}

	#[inline]
	fn read_u128_le(&mut self) -> Result<u128> {
		Ok(u128::from_le_bytes(read_bytes!(self, 16)))
	}
	#[inline]
	fn read_u128_be(&mut self) -> Result<u128> {
		Ok(u128::from_be_bytes(read_bytes!(self, 16)))
	}
}
pub trait ReadIntLiteral: Read {
	#[inline]
	fn read_i8(&mut self) -> Result<i8> {
		Ok(read_bytes!(self, 1)[0] as i8)
	}

	#[inline]
	fn read_i16_le(&mut self) -> Result<i16> {
		Ok(i16::from_le_bytes(read_bytes!(self, 2)))
	}
	#[inline]
	fn read_i16_be(&mut self) -> Result<i16> {
		Ok(i16::from_be_bytes(read_bytes!(self, 2)))
	}

	#[inline]
	fn read_i32_le(&mut self) -> Result<i32> {
		Ok(i32::from_le_bytes(read_bytes!(self, 4)))
	}
	#[inline]
	fn read_i32_be(&mut self) -> Result<i32> {
		Ok(i32::from_be_bytes(read_bytes!(self, 4)))
	}

	#[inline]
	fn read_i64_le(&mut self) -> Result<i64> {
		Ok(i64::from_le_bytes(read_bytes!(self, 8)))
	}
	#[inline]
	fn read_i64_be(&mut self) -> Result<i64> {
		Ok(i64::from_be_bytes(read_bytes!(self, 8)))
	}

	#[inline]
	fn read_i128_le(&mut self) -> Result<i128> {
		Ok(i128::from_le_bytes(read_bytes!(self, 16)))
	}
	#[inline]
	fn read_i128_be(&mut self) -> Result<i128> {
		Ok(i128::from_be_bytes(read_bytes!(self, 16)))
	}
}
pub trait ReadFloatLiteral: Read {
	#[inline]
	fn read_f32_le(&mut self) -> Result<f32> {
		Ok(f32::from_le_bytes(read_bytes!(self, 4)))
	}
	#[inline]
	fn read_f32_be(&mut self) -> Result<f32> {
		Ok(f32::from_be_bytes(read_bytes!(self, 4)))
	}

	#[inline]
	fn read_f64_le(&mut self) -> Result<f64> {
		Ok(f64::from_le_bytes(read_bytes!(self, 8)))
	}
	#[inline]
	fn read_f64_be(&mut self) -> Result<f64> {
		Ok(f64::from_be_bytes(read_bytes!(self, 8)))
	}
}

impl<R: Read> ReadUIntLiteral for R {}
impl<R: Read> ReadIntLiteral for R {}
impl<R: Read> ReadFloatLiteral for R {}

pub trait WriteUIntLiteral: Write {
	#[inline]
	fn write_u8(&mut self, n: u8) -> Result<()> {
		self.write_all(&[n])
	}

	#[inline]
	fn write_u16_le(&mut self, n: u16) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_u16_be(&mut self, n: u16) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}

	#[inline]
	fn write_u32_le(&mut self, n: u32) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_u32_be(&mut self, n: u32) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}

	#[inline]
	fn write_u64_le(&mut self, n: u64) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_u64_be(&mut self, n: u64) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}

	#[inline]
	fn write_u128_le(&mut self, n: u128) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_u128_be(&mut self, n: u128) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}
}
pub trait WriteIntLiteral: Write {
	#[inline]
	fn write_i8(&mut self, n: i8) -> Result<()> {
		self.write_all(&[n as u8])
	}

	#[inline]
	fn write_i16_le(&mut self, n: i16) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_i16_be(&mut self, n: i16) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}

	#[inline]
	fn write_i32_le(&mut self, n: i32) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_i32_be(&mut self, n: i32) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}

	#[inline]
	fn write_i64_le(&mut self, n: i64) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_i64_be(&mut self, n: i64) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}

	#[inline]
	fn write_i128_le(&mut self, n: i128) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_i128_be(&mut self, n: i128) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}
}
pub trait WriteFloatLiteral: Write {
	#[inline]
	fn write_f32_le(&mut self, n: f32) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_f32_be(&mut self, n: f32) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}

	#[inline]
	fn write_f64_le(&mut self, n: f64) -> Result<()> {
		self.write_all(&n.to_le_bytes())
	}
	#[inline]
	fn write_f64_be(&mut self, n: f64) -> Result<()> {
		self.write_all(&n.to_be_bytes())
	}
}

impl<W: Write> WriteUIntLiteral for W {}
impl<W: Write> WriteIntLiteral for W {}
impl<W: Write> WriteFloatLiteral for W {}
