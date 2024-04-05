use std::fs::File;

use formatic::{Arch, BinFormat, Decl, Endian, ObjectBuilder};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("test.o")?;
    let mut obj = ObjectBuilder::new();

    obj.decls(
        vec![
            ("callme", Decl::FunctionImport),
            ("call", Decl::FunctionExport)
        ]
    );

    obj.define("call", vec![
        0xF3, 0x0F, 0x1E, 0xFA,         // endbr64        
        0x55,                           // push rbp
        0x48, 0x89, 0xE5,               // mov rbp, rsp
        0xE8, 0x00, 0x00, 0x00, 0x00,   // call callme
        0x90,                           // nop
        0x5D,                           // pop rbp
        0xC3,                           // ret
    ]);

    obj.write(BinFormat::host(), Arch::host(), Endian::host(), file)?;

    Ok(())
}