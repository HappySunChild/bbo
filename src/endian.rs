pub mod runtime {
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

	#[derive(Clone, Copy)]
	pub enum Endianness {
		Little,
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
}
