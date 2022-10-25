use super::kind::TokenKind;
use crate::datatype::DataUnion;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Token {
    pub tk: TokenKind,
    pub data: DataUnion,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, kind:{:?}]", self.data, self.tk)
    }
}

#[allow(dead_code)]
impl Token {
    pub fn new(tk: TokenKind, data: DataUnion) -> Self {
        Token { tk, data }
    }

    pub fn with_str(tk: TokenKind, word: &str) -> Self {
        Token {
            tk,
            data: DataUnion::String(word.chars().collect()),
        }
    }

    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.tk == kind
    }

    pub fn eq_data(&self, data: DataUnion) -> bool {
        self.data == data
    }

    pub fn get_data(&self) -> &DataUnion {
        &self.data
    }
}
