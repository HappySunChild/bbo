use std::{
	fs::File,
	io::{BufWriter, Result as IoResult},
};

use bbo::ext::WriteBytesExt;

fn main() -> IoResult<()> {
	let file = File::create("hi.txt")?;
	let mut buf = BufWriter::new(file);

	buf.write_u32_le(0x01_02)?;

	Ok(())
}
