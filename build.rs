use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use std::io;

pub fn main() -> io::Result<()> {
  let bigrs_path = Path::new("./src/big.rs");
  if !bigrs_path.exists() {
    let data = File::open("./data")?;
    let mut bigrs = File::create(bigrs_path)?;
    write!(bigrs, "pub static BIG_DOB: &[u8] = &[")?;
    for byte in data.bytes() {
      write!(bigrs, "0x{:2x},", byte?)?;
    }
    writeln!(bigrs, "];")?;
  }
  Ok(())
}