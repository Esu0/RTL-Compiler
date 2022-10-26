use super::Node;
use crate::error::Error;
use crate::tokenize::{TokenGen, TokenKind};
use std::collections::HashMap;
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

    pub fn compile(
        &self,
        var: &mut HashMap<Vec<char>, usize>,
        mem: &mut Vec<i32>,
    ) -> Result<(), Error> {
        for n in &self.stmts {
            n.compile(var, mem)?;
        }
        Ok(())
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
