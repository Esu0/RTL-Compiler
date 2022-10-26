use super::Node;
use crate::error::Error;
use crate::tokenize::{TokenGen, TokenKind};
pub struct Program {
    stmts: Vec<Node>,
}

impl Program {
    pub fn from_tokens(token: &mut TokenGen) -> Result<Self, Error> {
        let mut nodes = Vec::new();
        while !token.current().is_kind(TokenKind::Eof) {
            nodes.push(Node::stmt(token)?);
        }
        Ok(Self { stmts: nodes })
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tr) in self.stmts.iter().enumerate() {
            writeln!(f, "{}th statement:\n{:?}", i, tr)?;
        }
        Ok(())
    }
}
