/// Contains functions for writing and reading little-endian encoded numbers to a buffer.
pub mod little {
	/// Reads a little-endian [`u16`] from the buffer.
	#[inline]
	pub fn read_u16(buf: &[u8]) -> u16 {
		u16::from_le_bytes(buf[..2].try_into().unwrap())
	}
	/// Reads a little-endian [`u32`] from the buffer.
	#[inline]
	pub fn read_u32(buf: &[u8]) -> u32 {
		u32::from_le_bytes(buf[..4].try_into().unwrap())
	}
	/// Reads a little-endian [`u64`] from the buffer.
	#[inline]
	pub fn read_u64(buf: &[u8]) -> u64 {
		u64::from_le_bytes(buf[..8].try_into().unwrap())
	}
	/// Reads a little-endian [`u128`] from the buffer.
	#[inline]
	pub fn read_u128(buf: &[u8]) -> u128 {
		u128::from_le_bytes(buf[..16].try_into().unwrap())
	}
	/// Reads a little-endian, unsigned n-byte integer from the buffer as a [`u64`].
	#[inline]
	pub fn read_uint64(buf: &[u8], byte_count: usize) -> u64 {
		assert!(byte_count <= 8);
		assert!(byte_count <= buf.len());

		let mut out = [0u8; 8];
		out[..byte_count].copy_from_slice(&buf[..byte_count]);
		u64::from_le_bytes(out)
	}
	/// Reads a little-endian, unsigned n-byte integer from the buffer as a [`u128`].
	#[inline]
	pub fn read_uint128(buf: &[u8], byte_count: usize) -> u128 {
		assert!(byte_count <= 16);
		assert!(byte_count <= buf.len());

		let mut out = [0u8; 16];
		out[..byte_count].copy_from_slice(&buf[..byte_count]);
		u128::from_le_bytes(out)
	}

	/// Writes a little-endian [`u16`] to the buffer.
	#[inline]
	pub fn write_u16(buf: &mut [u8], n: u16) {
		buf[..2].copy_from_slice(&n.to_le_bytes());
	}
	/// Writes a little-endian [`u32`] to the buffer.
	#[inline]
	pub fn write_u32(buf: &mut [u8], n: u32) {
		buf[..4].copy_from_slice(&n.to_le_bytes());
	}
	/// Writes a little-endian [`u64`] to the buffer.
	#[inline]
	pub fn write_u64(buf: &mut [u8], n: u64) {
		buf[..8].copy_from_slice(&n.to_le_bytes());
	}
	/// Writes a little-endian [`u128`] to the buffer.
	#[inline]
	pub fn write_u128(buf: &mut [u8], n: u128) {
		buf[..16].copy_from_slice(&n.to_le_bytes());
	}
	/// Writes a little-endian, unsigned n-byte integer as a [`u64`] to the buffer.
	#[inline]
	pub fn write_uint64(buf: &mut [u8], n: u64, byte_count: usize) {
		assert!(byte_count <= 8);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_le_bytes()[..byte_count]);
	}
	/// Writes a little-endian, unsigned n-byte integer as a [`u128`] to the buffer.
	#[inline]
	pub fn write_uint128(buf: &mut [u8], n: u128, byte_count: usize) {
		assert!(byte_count <= 16);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_le_bytes()[..byte_count]);
	}

	/// Reads a little-endian [`i16`] from the buffer.
	#[inline]
	pub fn read_i16(buf: &[u8]) -> i16 {
		read_u16(buf) as i16
	}
	/// Reads a little-endian [`i32`] from the buffer.
	#[inline]
	pub fn read_i32(buf: &[u8]) -> i32 {
		read_u32(buf) as i32
	}
	/// Reads a little-endian [`i64`] from the buffer.
	#[inline]
	pub fn read_i64(buf: &[u8]) -> i64 {
		read_u64(buf) as i64
	}
	/// Reads a little-endian [`i128`] from the buffer.
	#[inline]
	pub fn read_i128(buf: &[u8]) -> i128 {
		read_u128(buf) as i128
	}
	/// Reads a little-endian, signed n-byte integer from the buffer as a [`i64`].
	#[inline]
	pub fn read_int64(buf: &[u8], byte_count: usize) -> i64 {
		let unsigned = read_uint64(buf, byte_count);
		let shift_by = 64 - byte_count * 8;
		((unsigned << shift_by) as i64) >> shift_by
	}
	/// Reads a little-endian, signed n-byte integer from the buffer as a [`i128`].
	#[inline]
	pub fn read_int128(buf: &[u8], byte_count: usize) -> i128 {
		let unsigned = read_uint128(buf, byte_count);
		let shift_by = 128 - byte_count * 8;
		((unsigned << shift_by) as i128) >> shift_by
	}

	/// Writes a little-endian [`i16`] to the buffer.
	#[inline]
	pub fn write_i16(buf: &mut [u8], n: i16) {
		write_u16(buf, n as u16);
	}
	/// Writes a little-endian [`i32`] to the buffer.
	#[inline]
	pub fn write_i32(buf: &mut [u8], n: i32) {
		write_u32(buf, n as u32);
	}
	/// Writes a little-endian [`i64`] to the buffer.
	#[inline]
	pub fn write_i64(buf: &mut [u8], n: i64) {
		write_u64(buf, n as u64);
	}
	/// Writes a little-endian [`i128`] to the buffer.
	#[inline]
	pub fn write_i128(buf: &mut [u8], n: i128) {
		write_u128(buf, n as u128);
	}
	/// Writes a little-endian, signed n-byte integer as a [`i64`] to the buffer.
	#[inline]
	pub fn write_int64(buf: &mut [u8], n: i64, byte_count: usize) {
		write_uint64(buf, n as u64, byte_count);
	}
	/// Writes a little-endian, signed n-byte integer as a [`i128`] to the buffer.
	#[inline]
	pub fn write_int128(buf: &mut [u8], n: i128, byte_count: usize) {
		write_uint128(buf, n as u128, byte_count);
	}

	/// Reads a little-endian [`f32`] from the buffer.
	#[inline]
	pub fn read_f32(buf: &[u8]) -> f32 {
		f32::from_bits(read_u32(buf))
	}
	/// Reads a little-endian [`f64`] from the buffer.
	#[inline]
	pub fn read_f64(buf: &[u8]) -> f64 {
		f64::from_bits(read_u64(buf))
	}

	/// Writes a little-endian [`f32`] to the buffer.
	#[inline]
	pub fn write_f32(buf: &mut [u8], n: f32) {
		write_u32(buf, n.to_bits());
	}
	/// Writes a little-endian [`f64`] to the buffer.
	#[inline]
	pub fn write_f64(buf: &mut [u8], n: f64) {
		write_u64(buf, n.to_bits());
	}
}
/// Contains functions for writing and reading big-endian encoded numbers to a buffer.
pub mod big {
	/// Reads a big-endian [`u16`] from the buffer.
	#[inline]
	pub fn read_u16(buf: &[u8]) -> u16 {
		u16::from_be_bytes(buf[..2].try_into().unwrap())
	}
	/// Reads a big-endian [`u32`] from the buffer.
	#[inline]
	pub fn read_u32(buf: &[u8]) -> u32 {
		u32::from_be_bytes(buf[..4].try_into().unwrap())
	}
	/// Reads a big-endian [`u64`] from the buffer.
	#[inline]
	pub fn read_u64(buf: &[u8]) -> u64 {
		u64::from_be_bytes(buf[..8].try_into().unwrap())
	}
	/// Reads a big-endian [`u128`] from the buffer.
	#[inline]
	pub fn read_u128(buf: &[u8]) -> u128 {
		u128::from_be_bytes(buf[..16].try_into().unwrap())
	}
	/// Reads a big-endian, unsigned n-byte integer from the buffer as a [`u64`].
	#[inline]
	pub fn read_uint64(buf: &[u8], byte_count: usize) -> u64 {
		let mut out = [0u8; 8];
		out[8 - byte_count..].copy_from_slice(&buf[..byte_count]);
		u64::from_be_bytes(out)
	}
	/// Reads a big-endian, unsigned n-byte integer from the buffer as a [`u128`].
	#[inline]
	pub fn read_uint128(buf: &[u8], byte_count: usize) -> u128 {
		let mut out = [0u8; 16];
		out[16 - byte_count..].copy_from_slice(&buf[..byte_count]);
		u128::from_be_bytes(out)
	}

	/// Writes a big-endian [`u16`] to the buffer.
	#[inline]
	pub fn write_u16(buf: &mut [u8], n: u16) {
		buf[..2].copy_from_slice(&n.to_be_bytes());
	}
	/// Writes a big-endian [`u32`] to the buffer.
	#[inline]
	pub fn write_u32(buf: &mut [u8], n: u32) {
		buf[..4].copy_from_slice(&n.to_be_bytes());
	}
	/// Writes a big-endian [`u64`] to the buffer.
	#[inline]
	pub fn write_u64(buf: &mut [u8], n: u64) {
		buf[..8].copy_from_slice(&n.to_be_bytes());
	}
	/// Writes a big-endian [`u128`] to the buffer.
	#[inline]
	pub fn write_u128(buf: &mut [u8], n: u128) {
		buf[..16].copy_from_slice(&n.to_be_bytes());
	}
	/// Writes a big-endian, unsigned n-byte integer as a [`u64`] to the buffer.
	#[inline]
	pub fn write_uint64(buf: &mut [u8], n: u64, byte_count: usize) {
		assert!(byte_count <= 8);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_be_bytes()[8 - byte_count..]);
	}
	/// Writes a big-endian, unsigned n-byte integer as a [`u128`] to the buffer.
	#[inline]
	pub fn write_uint128(buf: &mut [u8], n: u128, byte_count: usize) {
		assert!(byte_count <= 16);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_be_bytes()[16 - byte_count..]);
	}

	/// Reads a big-endian [`i16`] from the buffer.
	#[inline]
	pub fn read_i16(buf: &[u8]) -> i16 {
		read_u16(buf) as i16
	}
	/// Reads a big-endian [`i32`] from the buffer.
	#[inline]
	pub fn read_i32(buf: &[u8]) -> i32 {
		read_u32(buf) as i32
	}
	/// Reads a big-endian [`i64`] from the buffer.
	#[inline]
	pub fn read_i64(buf: &[u8]) -> i64 {
		read_u64(buf) as i64
	}
	/// Reads a big-endian [`i128`] from the buffer.
	#[inline]
	pub fn read_i128(buf: &[u8]) -> i128 {
		read_u128(buf) as i128
	}
	/// Reads a big-endian, signed n-byte integer from the buffer as a [`i64`].
	#[inline]
	pub fn read_int64(buf: &[u8], byte_count: usize) -> i64 {
		let unsigned = read_uint64(buf, byte_count);
		let shift_by = 64 - byte_count * 8;
		((unsigned << shift_by) as i64) >> shift_by
	}
	/// Reads a big-endian, signed n-byte integer from the buffer as a [`i128`].
	#[inline]
	pub fn read_int128(buf: &[u8], byte_count: usize) -> i128 {
		let unsigned = read_uint128(buf, byte_count);
		let shift_by = 128 - byte_count * 8;
		((unsigned << shift_by) as i128) >> shift_by
	}

	/// Writes a big-endian [`i16`] to the buffer.
	#[inline]
	pub fn write_i16(buf: &mut [u8], n: i16) {
		write_u16(buf, n as u16);
	}
	/// Writes a big-endian [`i32`] to the buffer.
	#[inline]
	pub fn write_i32(buf: &mut [u8], n: i32) {
		write_u32(buf, n as u32);
	}
	/// Writes a big-endian [`i64`] to the buffer.
	#[inline]
	pub fn write_i64(buf: &mut [u8], n: i64) {
		write_u64(buf, n as u64);
	}
	/// Writes a big-endian [`i128`] to the buffer.
	#[inline]
	pub fn write_i128(buf: &mut [u8], n: i128) {
		write_u128(buf, n as u128);
	}
	/// Writes a big-endian, signed n-byte integer as a [`i64`] to the buffer.
	#[inline]
	pub fn write_int64(buf: &mut [u8], n: i64, byte_count: usize) {
		write_uint64(buf, n as u64, byte_count);
	}
	/// Writes a big-endian, signed n-byte integer as a [`i128`] to the buffer.
	#[inline]
	pub fn write_int128(buf: &mut [u8], n: i128, byte_count: usize) {
		write_uint128(buf, n as u128, byte_count);
	}

	/// Reads a big-endian [`f32`] from the buffer.
	#[inline]
	pub fn read_f32(buf: &[u8]) -> f32 {
		f32::from_bits(read_u32(buf))
	}
	/// Reads a big-endian [`f64`] from the buffer.
	#[inline]
	pub fn read_f64(buf: &[u8]) -> f64 {
		f64::from_bits(read_u64(buf))
	}

	/// Writes a big-endian [`f32`] to the buffer.
	#[inline]
	pub fn write_f32(buf: &mut [u8], n: f32) {
		write_u32(buf, n.to_bits());
	}
	/// Writes a big-endian [`f64`] to the buffer.
	#[inline]
	pub fn write_f64(buf: &mut [u8], n: f64) {
		write_u64(buf, n.to_bits());
	}
}

/// Reads a [`u8`] from the buffer.
#[inline]
pub fn read_u8(buf: &[u8]) -> u8 {
	buf[0]
}
/// Writes a [`u8`] to the buffer.
#[inline]
pub fn write_u8(buf: &mut [u8], n: u8) {
	buf[0] = n
}

/// Reads a [`i8`] from the buffer.
#[inline]
pub fn read_i8(buf: &[u8]) -> i8 {
	buf[0] as i8
}
/// Writes a [`i8`] to the buffer.
#[inline]
pub fn write_i8(buf: &mut [u8], n: i8) {
	buf[0] = n as u8
}
