use std::io::Result as IOResult;

pub trait ReadBytes {
	fn read_u8(&mut self) -> IOResult<u8>;
	fn read_u16_le(&mut self) -> IOResult<u16>;
	fn read_u32_le(&mut self) -> IOResult<u32>;
	fn read_u64_le(&mut self) -> IOResult<u64>;
	fn read_u128_le(&mut self) -> IOResult<u128>;

	#[inline]
	fn read_u16_be(&mut self) -> IOResult<u16> {
		Ok(self.read_u16_le()?.swap_bytes())
	}
	#[inline]
	fn read_u32_be(&mut self) -> IOResult<u32> {
		Ok(self.read_u32_le()?.swap_bytes())
	}
	#[inline]
	fn read_u64_be(&mut self) -> IOResult<u64> {
		Ok(self.read_u64_le()?.swap_bytes())
	}
	#[inline]
	fn read_u128_be(&mut self) -> IOResult<u128> {
		Ok(self.read_u128_le()?.swap_bytes())
	}
}
pub trait ReadBytesSigned: ReadBytes {
	#[inline]
	fn read_i8(&mut self) -> IOResult<i8> {
		Ok(self.read_u8()? as i8)
	}

	#[inline]
	fn read_i16_le(&mut self) -> IOResult<i16> {
		Ok(self.read_u16_le()? as i16)
	}
	#[inline]
	fn read_i16_be(&mut self) -> IOResult<i16> {
		Ok(self.read_u16_be()? as i16)
	}

	#[inline]
	fn read_i32_le(&mut self) -> IOResult<i32> {
		Ok(self.read_u32_le()? as i32)
	}
	#[inline]
	fn read_i32_be(&mut self) -> IOResult<i32> {
		Ok(self.read_u32_be()? as i32)
	}

	#[inline]
	fn read_i64_le(&mut self) -> IOResult<i64> {
		Ok(self.read_u64_le()? as i64)
	}
	#[inline]
	fn read_i64_be(&mut self) -> IOResult<i64> {
		Ok(self.read_u64_be()? as i64)
	}

	#[inline]
	fn read_i128_le(&mut self) -> IOResult<i128> {
		Ok(self.read_u128_le()? as i128)
	}
	#[inline]
	fn read_i128_be(&mut self) -> IOResult<i128> {
		Ok(self.read_u128_be()? as i128)
	}
}
pub trait ReadBytesFloat: ReadBytes {
	#[inline]
	fn read_f32_le(&mut self) -> IOResult<f32> {
		Ok(f32::from_bits(self.read_u32_le()?))
	}
	#[inline]
	fn read_f32_be(&mut self) -> IOResult<f32> {
		Ok(f32::from_bits(self.read_u32_be()?))
	}

	#[inline]
	fn read_f64_le(&mut self) -> IOResult<f64> {
		Ok(f64::from_bits(self.read_u64_le()?))
	}
	#[inline]
	fn read_f64_be(&mut self) -> IOResult<f64> {
		Ok(f64::from_bits(self.read_u64_be()?))
	}
}

pub trait WriteBytes {
	fn write_u8(&mut self, n: u8) -> IOResult<()>;
	fn write_u16_le(&mut self, n: u16) -> IOResult<()>;
	fn write_u32_le(&mut self, n: u32) -> IOResult<()>;
	fn write_u64_le(&mut self, n: u64) -> IOResult<()>;
	fn write_u128_le(&mut self, n: u128) -> IOResult<()>;

	#[inline]
	fn write_u16_be(&mut self, n: u16) -> IOResult<()> {
		self.write_u16_le(n.swap_bytes())
	}
	#[inline]
	fn write_u32_be(&mut self, n: u32) -> IOResult<()> {
		self.write_u32_le(n.swap_bytes())
	}
	#[inline]
	fn write_u64_be(&mut self, n: u64) -> IOResult<()> {
		self.write_u64_le(n.swap_bytes())
	}
	#[inline]
	fn write_u128_be(&mut self, n: u128) -> IOResult<()> {
		self.write_u128_le(n.swap_bytes())
	}
}
pub trait WriteBytesSigned: WriteBytes {
	#[inline]
	fn write_i8(&mut self, n: i8) -> IOResult<()> {
		self.write_u8(n as u8)
	}

	#[inline]
	fn write_i16_le(&mut self, n: i16) -> IOResult<()> {
		self.write_u16_le(n as u16)
	}
	#[inline]
	fn write_i16_be(&mut self, n: i16) -> IOResult<()> {
		self.write_u16_be(n as u16)
	}

	#[inline]
	fn write_i32_le(&mut self, n: i32) -> IOResult<()> {
		self.write_u32_le(n as u32)
	}
	#[inline]
	fn write_i32_be(&mut self, n: i32) -> IOResult<()> {
		self.write_u32_be(n as u32)
	}

	#[inline]
	fn write_i64_le(&mut self, n: i64) -> IOResult<()> {
		self.write_u64_le(n as u64)
	}
	#[inline]
	fn write_i64_be(&mut self, n: i64) -> IOResult<()> {
		self.write_u64_be(n as u64)
	}

	#[inline]
	fn write_i128_le(&mut self, n: i128) -> IOResult<()> {
		self.write_u128_le(n as u128)
	}
	#[inline]
	fn write_i128_be(&mut self, n: i128) -> IOResult<()> {
		self.write_u128_be(n as u128)
	}
}
pub trait WriteBytesFloat: WriteBytes {
	fn write_f32_le(&mut self, n: f32) -> IOResult<()> {
		self.write_u32_le(n.to_bits())
	}
	fn write_f32_be(&mut self, n: f32) -> IOResult<()> {
		self.write_u32_be(n.to_bits())
	}

	fn write_f64_le(&mut self, n: f64) -> IOResult<()> {
		self.write_u64_le(n.to_bits())
	}
	fn write_f64_be(&mut self, n: f64) -> IOResult<()> {
		self.write_u64_be(n.to_bits())
	}
}

pub trait ReadBytesArch {
	fn read_usize(&mut self) -> IOResult<usize>;
	fn read_isize(&mut self) -> IOResult<isize>;
}
pub trait WriteBytesArch {
	fn write_usize(&mut self) -> IOResult<()>;
	fn write_isize(&mut self) -> IOResult<()>;
}
