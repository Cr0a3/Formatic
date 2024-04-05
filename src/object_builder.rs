use std::{collections::HashMap, fs::File};

use crate::{Decl, Link, ObjectError};

/// Enum which specifies the binary format
/// 
/// E.g: Coff for Windows
pub enum BinFormat {
    Elf,
    Coff,
    Macho,
}

impl BinFormat {
    /// Function which returns the native binary format
    /// 
    /// For any unknown os it returns elf
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

/// Enum which specifies the architecture
pub enum Arch {
    X86_64,
    Arm,
    Riscv32,
    Riscv64,
    Wasm32,
    Wasm64,
}

impl Arch {
    /// Returns the native architecture
    pub fn Host() -> Arch {
        if cfg!(target_arch = "x86_64") {
            Arch::X86_64
        } else if cfg!(target_arch = "arm") {
            Arch::Arm
        } else if cfg!(target_arch = "riscv") {
            Arch::Riscv32
        }
    }
}

/// Enum which specifies the endiannes 
pub enum Endian {
    Litte,
    Big,
}

impl Endian {
    /// Returns the native endian
    pub fn Host() -> Endian {
        if cfg!(target_endian = "big") {
            Endian::Big
        } else {
            Endian::Litte
        }

    }
}

/// A struct for building object files
pub struct ObjectBuilder {
    decls: Vec<(String, Decl)>,
    sym: HashMap<String, Vec<u8>>,
    links: Vec<Link>,
}

impl ObjectBuilder {
    //// Returns empty instance of self
    pub fn new() -> Self {
        Self {
            decls: vec![],
            sym: HashMap::new(),
            links: vec![],
        }
    }

    /// Adds a list of decls
    pub fn decls(&mut self, decls: Vec<(String, Decl)>) {
        for decl in decls {
            self.decls.push(decl);
        }
    }

    /// Adds a decl
    pub fn add_decl(&mut self, name: String, decl: Decl) {
        self.decls.push((name, decl));
    }

    /// Defines a symbol
    pub fn define(&mut self, sym: String, data: Vec<u8>) {
        self.sym.insert(sym, data);
    }

    /// Adds an link to the object file
    pub fn link(&mut self, link: Link) {
        self.links.push(link);
    }

    /// Writes all internaly saved symbols etc. to a object file
    /// 
    /// Args:
    ///  * `format`   - specifes the binary format of the object file
    ///  * `arch`     - specifes the architecture of the object file
    ///  * `endian`   - specifes the endian of the object file
    ///  * `file`     - the file to write all of that
    pub fn write(
        &mut self, format: BinFormat, arch: Arch, endian: Endian, file: File
    ) -> Result<(), Box<dyn std::error::Error>> {
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
