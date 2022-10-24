use super::Node;
use crate::error::Error;
use crate::tokenize::{TokenKind, TokenList};
pub struct Program {
    stmts: Vec<Node>,
}

impl Program {
    pub fn from_tokens(token: &mut TokenList) -> Result<Self, Error> {
        let mut nodes = Vec::new();
        while token.current() != TokenKind::Eof {
            nodes.push(match Node::stmt(token) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            });
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
