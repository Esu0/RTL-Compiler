#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd)]
#[allow(dead_code)]
pub enum TokenKind {
    Number,
    Reserved,
    Ident,
    Eof,
}
