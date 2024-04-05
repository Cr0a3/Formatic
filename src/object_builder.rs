use std::{collections::HashMap, fs::File};

use crate::{Decl, Link, ObjectError};

pub enum BinFormat {
    Elf,
    Coff,
    Macho,
}

impl BinFormat {
    pub fn Host() -> BinFormat {
        if cfg!(target_os = "windows") {
            BinFormat::Coff
        } else if cfg!(target_os = "macos") {
            BinFormat::Macho
        } else {
            BinFormat::Elf
        }  
    }
}

pub enum Arch {
    X86_64,
    Arm,
    Riscv32,
    Riscv64,
    Wasm32,
    Wasm64,
}

pub enum Endian {
    Litte,
    Big,
}

pub struct ObjectBuilder {
    decls: Vec<(String, Decl)>,
    sym: HashMap<String, Vec<u8>>,
    links: Vec<Link>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            decls: vec![],
            sym: HashMap::new(),
            links: vec![],
        }
    }

    pub fn decls(&mut self, decls: Vec<(String, Decl)>) {
        for decl in decls {
            self.decls.push(decl);
        }
    }

    pub fn add_decl(&mut self, name: String, decl: Decl) {
        self.decls.push((name, decl));
    }

    pub fn define(&mut self, sym: String, data: Vec<u8>) {
        self.sym.insert(sym, data);
    }

    pub fn link(&mut self, link: Link) {
        self.links.push(link);
    }

    pub fn write(&mut self, format: BinFormat, arch: Arch, endian: Endian, file: File) -> Result<(), Box<dyn std::error::Error>> {
        let obj_format = match format {
            BinFormat::Elf => object::BinaryFormat::Elf,
            BinFormat::Coff => object::BinaryFormat::Coff,
            BinFormat::Macho => object::BinaryFormat::MachO,
        };

        let obj_arch = match arch {
            Arch::X86_64 => object::Architecture::X86_64,
            Arch::Arm => object::Architecture::Arm,
            Arch::Riscv32 => object::Architecture::Riscv32,
            Arch::Riscv64 => object::Architecture::Riscv64,
            Arch::Wasm32 => object::Architecture::Wasm32,
            Arch::Wasm64 => object::Architecture::Wasm64,
        };

        let obj_endian = match endian {
            Endian::Litte => object::Endianness::Little,
            Endian::Big => object::Endianness::Big,
        };

        let mut obj = object::write::Object::new(obj_format, obj_arch, obj_endian);

        obj.write_stream(file)?;

        Ok(())
    }
}