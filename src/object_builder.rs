use std::{collections::HashMap, fs::File};

use object::{write::{Relocation, SectionId, StandardSection, Symbol, SymbolId, SymbolSection}, RelocationEncoding, RelocationFlags, RelocationKind, SymbolFlags, SymbolKind, SymbolScope};

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
    pub fn host() -> BinFormat {
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
    Unknown,
}

impl Arch {
    /// Returns the native architecture
    pub fn host() -> Arch {
        if cfg!(target_arch = "x86_64") {
            Arch::X86_64
        } else if cfg!(target_arch = "arm") {
            Arch::Arm
        } else if cfg!(target_arch = "riscv32") {
            Arch::Riscv32
        } else if cfg!(target_arch = "riscv64") {
            Arch::Riscv64
        } else if cfg!(target_arch = "wasm32") {
            Arch::Wasm32
        } else if cfg!(target_arch = "wasm64") {
            Arch::Wasm32
        } else { Arch::Unknown }
    }
}

/// Enum which specifies the endiannes 
pub enum Endian {
    Litte,
    Big,
}

impl Endian {
    /// Returns the native endian
    pub fn host() -> Endian {
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
    pub fn decls(&mut self, decls: Vec<(&str, Decl)>) {
        for decl in decls {
            let decl = (decl.0.into(), decl.1);
            self.decls.push(decl);
        }
    }

    /// Adds a decl
    pub fn add_decl(&mut self, name: &str, decl: Decl) {
        self.decls.push((name.into(), decl));
    }

    /// Defines a symbol
    pub fn define(&mut self, sym: &str, data: Vec<u8>) {
        self.sym.insert(sym.into(), data);
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
            Arch::Unknown => object::Architecture::Unknown,
        };

        let obj_endian = match endian {
            Endian::Litte => object::Endianness::Little,
            Endian::Big => object::Endianness::Big,
        };

        let mut obj = object::write::Object::new(obj_format, obj_arch, obj_endian);

        let mut ids: HashMap<String, SymbolId> = HashMap::new();
        let mut funcs: HashMap<String, ((SectionId, u64), SymbolId)> = HashMap::new();

        for decl in self.decls.iter() {
            let name = &decl.0;
            let decl = &decl.1;

            // get type
            match decl {
                Decl::FunctionImport => {
                    ids.insert(name.to_string(),
                            obj.add_symbol(Symbol {
                            name: name.as_bytes().into(),
                            value: 0,
                            size: 0,
                            kind: SymbolKind::Text,
                            scope: SymbolScope::Dynamic,
                            weak: false,
                            section: SymbolSection::Undefined,
                            flags: SymbolFlags::None,
                            })
                    );
                }

                Decl::DataImport => {
                    ids.insert(name.to_string(),
                            obj.add_symbol(Symbol {
                            name: name.as_bytes().into(),
                            value: 0,
                            size: 0,
                            kind: SymbolKind::Data,
                            scope: SymbolScope::Dynamic,
                            weak: false,
                            section: SymbolSection::Undefined,
                            flags: SymbolFlags::None,
                            })
                    );
                }

                Decl::FunctionExport => {
                    let dat_opt = self.sym.get(&name.clone());

                    if dat_opt.is_none() {
                        return Err( Box::from(ObjectError::DeclWithoutSymbol) );
                    }

                    let data = dat_opt.unwrap();

                    let (section, offset) =
                        obj.add_subsection(StandardSection::Text, name.as_bytes().into(), data, 16);
                    let symbol = obj.add_symbol(Symbol {
                        name: name.as_bytes().into(),
                        value: offset,
                        size: data.len() as u64,
                        kind: SymbolKind::Text,
                        scope: SymbolScope::Linkage,
                        weak: false,
                        section: SymbolSection::Section(section),
                        flags: SymbolFlags::None,
                    });

                    funcs.insert(name.into(), ((section, offset), symbol) );
                }
            }
        }

        for link in self.links.iter() {
            let link = link.to_owned();

            let func_opt = funcs.get(&link.from);
            if func_opt.is_none() {
                return Err( Box::from( ObjectError::UnknownFunction(link.from.to_owned()) ) );
            }
            let func = func_opt.unwrap();

            let id = func.0.0;
            let off = func.0.1;
            let sym = func.1;

            obj.add_relocation(
                id, 
                Relocation {
                    offset: off + link.at as u64,
                    symbol: sym,
                    addend: -4,
                    flags: RelocationFlags::Generic {
                        kind: RelocationKind::PltRelative,
                        encoding: RelocationEncoding::X86Branch,
                        size: 32,
                    },
                }
            )?;
        }

        obj.write_stream(file)?;

        Ok(())
    }
}
