pub enum Decl {
    Function(Scope),
    Data(Scope),
}

pub enum Scope {
    Import,
    Export,
}