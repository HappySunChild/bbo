# bufferbyteorder
A crate containing some useful traits for writing to and reading from buffers or streams more ergonomically.

## Usage
Here is a simple example of writing little endian encoded bytes to a file, using the traits from this crate.
```rs
use bbo::ext::WriteBytesExt;

fn main() -> std::io::Result<()> {
	let f = std::fs::File::create("example.txt")?;
	let mut buf = std::io::BufWriter::new(f);

	buf.write_u8(31_u8)?;
	buf.write_i16_le(31415_i16)?;
	buf.write_f64_le(3.134159_f64)?;

	Ok(())
}
```

> [!NOTE]
> This crate has traits that automatically implement methods for any objects that implement the `std::io::Read` and `std::io::Write` traits, namely the `WriteBytesExt` and `ReadBytesExt` traits respectively.