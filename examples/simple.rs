use std::fs::File;

use formatic::{Arch, BinFormat, Endian, ObjectBuilder};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let obj = ObjectBuilder::new();

    let file = File::create("test.o")?;
    obj.write(BinFormat::Host(), Arch::Host(), Endian::Host(), file)?;

    Ok(())
}