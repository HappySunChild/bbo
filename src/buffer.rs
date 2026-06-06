pub mod little {
	macro_rules! fn_read_bytes_le {
		($name:ident, $out:ty, $bytes:expr) => {
			#[inline]
			pub fn $name(buf: &[u8]) -> $out {
				<$out>::from_le_bytes(buf[..$bytes].try_into().unwrap())
			}
		};
	}
	macro_rules! fn_write_bytes_le {
		($name:ident, $in:ty, $bytes:expr) => {
			#[inline]
			pub fn $name(buf: &mut [u8], n: $in) {
				buf[..$bytes].copy_from_slice(&n.to_le_bytes());
			}
		};
	}

	fn_read_bytes_le!(read_u16, u16, 2);
	fn_read_bytes_le!(read_u32, u32, 4);
	fn_read_bytes_le!(read_u64, u64, 8);
	fn_read_bytes_le!(read_u128, u128, 16);
	#[inline]
	pub fn read_uint64(buf: &[u8], byte_count: usize) -> u64 {
		assert!(byte_count <= 8);
		assert!(byte_count <= buf.len());

		let mut out = [0u8; 8];
		out[..byte_count].copy_from_slice(&buf[..byte_count]);
		u64::from_le_bytes(out)
	}
	#[inline]
	pub fn read_uint128(buf: &[u8], byte_count: usize) -> u128 {
		assert!(byte_count <= 16);
		assert!(byte_count <= buf.len());

		let mut out = [0u8; 16];
		out[..byte_count].copy_from_slice(&buf[..byte_count]);
		u128::from_le_bytes(out)
	}

	fn_write_bytes_le!(write_u16, u16, 2);
	fn_write_bytes_le!(write_u32, u32, 4);
	fn_write_bytes_le!(write_u64, u64, 8);
	fn_write_bytes_le!(write_u128, u128, 16);
	#[inline]
	pub fn write_uint64(buf: &mut [u8], n: u64, byte_count: usize) {
		assert!(byte_count <= 8);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_le_bytes()[..byte_count]);
	}
	#[inline]
	pub fn write_uint128(buf: &mut [u8], n: u128, byte_count: usize) {
		assert!(byte_count <= 16);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_le_bytes()[..byte_count]);
	}

	#[inline]
	pub fn read_i16(buf: &[u8]) -> i16 {
		read_u16(buf) as i16
	}
	#[inline]
	pub fn read_i32(buf: &[u8]) -> i32 {
		read_u32(buf) as i32
	}
	#[inline]
	pub fn read_i64(buf: &[u8]) -> i64 {
		read_u64(buf) as i64
	}
	#[inline]
	pub fn read_i128(buf: &[u8]) -> i128 {
		read_u128(buf) as i128
	}
	#[inline]
	pub fn read_int64(buf: &[u8], byte_count: usize) -> i64 {
		let unsigned = read_uint64(buf, byte_count);
		let shift_by = 64 - byte_count * 8;
		((unsigned << shift_by) as i64) >> shift_by
	}
	#[inline]
	pub fn read_int128(buf: &[u8], byte_count: usize) -> i128 {
		let unsigned = read_uint128(buf, byte_count);
		let shift_by = 128 - byte_count * 8;
		((unsigned << shift_by) as i128) >> shift_by
	}

	#[inline]
	pub fn write_i16(buf: &mut [u8], n: i16) {
		write_u16(buf, n as u16);
	}
	#[inline]
	pub fn write_i32(buf: &mut [u8], n: i32) {
		write_u32(buf, n as u32);
	}
	#[inline]
	pub fn write_i64(buf: &mut [u8], n: i64) {
		write_u64(buf, n as u64);
	}
	#[inline]
	pub fn write_i128(buf: &mut [u8], n: i128) {
		write_u128(buf, n as u128);
	}
	#[inline]
	pub fn write_int64(buf: &mut [u8], n: i64, byte_count: usize) {
		write_uint64(buf, n as u64, byte_count);
	}
	#[inline]
	pub fn write_int128(buf: &mut [u8], n: i128, byte_count: usize) {
		write_uint128(buf, n as u128, byte_count);
	}

	#[inline]
	pub fn read_f32(buf: &[u8]) -> f32 {
		f32::from_bits(read_u32(buf))
	}
	#[inline]
	pub fn read_f64(buf: &[u8]) -> f64 {
		f64::from_bits(read_u64(buf))
	}

	#[inline]
	pub fn write_f32(buf: &mut [u8], n: f32) {
		write_u32(buf, n.to_bits());
	}
	#[inline]
	pub fn write_f64(buf: &mut [u8], n: f64) {
		write_u64(buf, n.to_bits());
	}
}
pub mod big {
	macro_rules! fn_read_bytes_be {
		($name:ident, $out:ty, $bytes:expr) => {
			#[inline]
			pub fn $name(buf: &[u8]) -> $out {
				<$out>::from_be_bytes(buf[..$bytes].try_into().unwrap())
			}
		};
	}
	macro_rules! fn_write_bytes_be {
		($name:ident, $in:ty, $bytes:expr) => {
			#[inline]
			pub fn $name(buf: &mut [u8], n: $in) {
				buf[..$bytes].copy_from_slice(&n.to_be_bytes());
			}
		};
	}

	fn_read_bytes_be!(read_u16, u16, 2);
	fn_read_bytes_be!(read_u32, u32, 4);
	fn_read_bytes_be!(read_u64, u64, 8);
	fn_read_bytes_be!(read_u128, u128, 16);
	#[inline]
	pub fn read_uint64(buf: &[u8], byte_count: usize) -> u64 {
		let mut out = [0u8; 8];
		out[8 - byte_count..].copy_from_slice(&buf[..byte_count]);
		u64::from_be_bytes(out)
	}
	#[inline]
	pub fn read_uint128(buf: &[u8], byte_count: usize) -> u128 {
		let mut out = [0u8; 16];
		out[16 - byte_count..].copy_from_slice(&buf[..byte_count]);
		u128::from_be_bytes(out)
	}

	fn_write_bytes_be!(write_u16, u16, 2);
	fn_write_bytes_be!(write_u32, u32, 4);
	fn_write_bytes_be!(write_u64, u64, 8);
	fn_write_bytes_be!(write_u128, u128, 16);
	#[inline]
	pub fn write_uint64(buf: &mut [u8], n: u64, byte_count: usize) {
		assert!(byte_count <= 8);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_be_bytes()[8 - byte_count..]);
	}
	#[inline]
	pub fn write_uint128(buf: &mut [u8], n: u128, byte_count: usize) {
		assert!(byte_count <= 16);
		assert!(byte_count <= buf.len());

		buf[..byte_count].copy_from_slice(&n.to_be_bytes()[16 - byte_count..]);
	}

	#[inline]
	pub fn read_i16(buf: &[u8]) -> i16 {
		read_u16(buf) as i16
	}
	#[inline]
	pub fn read_i32(buf: &[u8]) -> i32 {
		read_u32(buf) as i32
	}
	#[inline]
	pub fn read_i64(buf: &[u8]) -> i64 {
		read_u64(buf) as i64
	}
	#[inline]
	pub fn read_i128(buf: &[u8]) -> i128 {
		read_u128(buf) as i128
	}
	#[inline]
	pub fn read_int64(buf: &[u8], byte_count: usize) -> i64 {
		let unsigned = read_uint64(buf, byte_count);
		let shift_by = 64 - byte_count * 8;
		((unsigned << shift_by) as i64) >> shift_by
	}
	#[inline]
	pub fn read_int128(buf: &[u8], byte_count: usize) -> i128 {
		let unsigned = read_uint128(buf, byte_count);
		let shift_by = 128 - byte_count * 8;
		((unsigned << shift_by) as i128) >> shift_by
	}

	#[inline]
	pub fn write_i16(buf: &mut [u8], n: i16) {
		write_u16(buf, n as u16);
	}
	#[inline]
	pub fn write_i32(buf: &mut [u8], n: i32) {
		write_u32(buf, n as u32);
	}
	#[inline]
	pub fn write_i64(buf: &mut [u8], n: i64) {
		write_u64(buf, n as u64);
	}
	#[inline]
	pub fn write_i128(buf: &mut [u8], n: i128) {
		write_u128(buf, n as u128);
	}
	#[inline]
	pub fn write_int64(buf: &mut [u8], n: i64, byte_count: usize) {
		write_uint64(buf, n as u64, byte_count);
	}
	#[inline]
	pub fn write_int128(buf: &mut [u8], n: i128, byte_count: usize) {
		write_uint128(buf, n as u128, byte_count);
	}

	#[inline]
	pub fn read_f32(buf: &[u8]) -> f32 {
		f32::from_bits(read_u32(buf))
	}
	#[inline]
	pub fn read_f64(buf: &[u8]) -> f64 {
		f64::from_bits(read_u64(buf))
	}

	#[inline]
	pub fn write_f32(buf: &mut [u8], n: f32) {
		write_u32(buf, n.to_bits());
	}
	#[inline]
	pub fn write_f64(buf: &mut [u8], n: f64) {
		write_u64(buf, n.to_bits());
	}
}

#[inline]
pub fn read_u8(buf: &[u8]) -> u8 {
	buf[0]
}
#[inline]
pub fn write_u8(buf: &mut [u8], n: u8) {
	buf[0] = n
}

#[inline]
pub fn read_i8(buf: &[u8]) -> i8 {
	buf[0] as i8
}
#[inline]
pub fn write_i8(buf: &mut [u8], n: i8) {
	buf[0] = n as u8
}
