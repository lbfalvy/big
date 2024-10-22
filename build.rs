use std::path::Path;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::io;

static DATA: &[u8; 48] = b"/rSznuIbdMlAKbO9/3ZYS/AhWflNjO2wDkOl6aJkegUKTaOY";

pub fn main() -> io::Result<()> {
  let bigrs_path = Path::new("./src/big.rs");
  if !bigrs_path.exists() {
    let mut bigrs = BufWriter::new(File::create(bigrs_path)?);
    write!(bigrs, "pub static BIG_DOB: &[u8] = &[")?;
    for i in 0..(48*1024*1024) {
      write!(bigrs, "0x{:x},", DATA[i % DATA.len()])?;
    }
    writeln!(bigrs, "];")?;
    bigrs.flush()?;
  }
  Ok(())
}