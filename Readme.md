# Formatic

A libary for creating object files

## Example

```rust
use formatic::{Arch, BinFormat, Decl, Endian, Link, ObjectBuilder, Scope};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut obj = ObjectBuilder::new("test.o");

    obj.decls(vec![
        ("callme", Decl::Function(Scope::Import)),
        ("call", Decl::Function(Scope::Export)),
    ]);

    obj.define(
        "call",
        vec![
            0xF3, 0x0F, 0x1E, 0xFA, // endbr64
            0x55, // push rbp
            0x48, 0x89, 0xE5, // mov rbp, rsp
            0xE8, 0x00, 0x00, 0x00, 0x00, // call callme
            0x90, // nop
            0x5D, // pop rbp
            0xC3, // ret
        ],
    );

    obj.link(Link {
        from: "call".into(),
        to: "callme".into(),
        at: 9,
    });

    obj.write(BinFormat::host(), Arch::host(), Endian::host())?;

    Ok(())
}

```

Which gives the expected output:
```bash
cargo run --example simple
bingrep test.o
```

```
ELF REL X86_64-little-endian @ 0x0:

e_phoff: 0x0 e_shoff: 0x150 e_flags: 0x0 e_ehsize: 64 e_phentsize: 0 e_phnum: 0 e_shentsize: 64 e_shnum: 7 e_shstrndx: 6

ProgramHeaders(0):
  Idx   Type   Flags   Offset   Vaddr   Paddr   Filesz   Memsz   Align  

SectionHeaders(7):
  Idx   Name                      Type   Flags              Offset   Addr   Size    Link         Entsize   Align  
  0                           SHT_NULL                      0x0      0x0    0x0                  0x0       0x0    
  1     .text.call        SHT_PROGBITS   ALLOC EXECINSTR    0x40     0x0    0x10                 0x0       0x10   
  2     .rela.text.call       SHT_RELA   INFO_LINK          0xf8     0x0    0x18    .symtab(4)   0x18      0x8    
  3     .rodata.value     SHT_PROGBITS   ALLOC              0x50     0x0    0xb                  0x0       0x10   
  4     .symtab             SHT_SYMTAB                      0x60     0x0    0x78    .strtab(5)   0x18      0x8    
  5     .strtab             SHT_STRTAB                      0xd8     0x0    0x1a                 0x0       0x1    
  6     .shstrtab           SHT_STRTAB                      0x110    0x0    0x39                 0x0       0x1    

Syms(5):
               Addr   Bind       Type        Symbol   Size    Section            Other  
                 0    LOCAL      NOTYPE               0x0                        0x0    
                 0    LOCAL      FILE        test.o   0x0     ABS                0x0    
                 0    GLOBAL     NOTYPE      callme   0x0                        0x0    
                 0    GLOBAL     FUNC        call     0x10    .text.call(1)      0x2    
                 0    GLOBAL     OBJECT      value    0xb     .rodata.value(3)   0x2    

Dyn Syms(0):
Dynamic Relas(0):

Dynamic Rel(0):

Plt Relocations(0):

Shdr Relocations(1):
  .text.call(1)
               9 X86_64_PLT32 callme+-4


Dynamic: None


Libraries(0):

Soname: None
Interpreter: None
is_64: true
is_lib: false
little_endian: true
entry: 0

```
