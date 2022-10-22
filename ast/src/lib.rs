use errors::{Error, ErrorKind};
use std::rc::Rc;
use token::{DataUnion, TokenList};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum Ntype {
    Num,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Node {
    ty: Ntype,
    childs: Option<Rc<(Node, Node)>>,
    value: DataUnion,
}

#[allow(dead_code)]
impl Ntype {
    fn has_childs(self) -> bool {
        if self > Ntype::Num {
            false
        } else {
            true
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_debug(f, 0)
    }
}

#[allow(dead_code)]
impl Node {
    pub fn new(ty: Ntype) -> Self {
        Self {
            ty,
            childs: None,
            value: match ty {
                Ntype::Num => DataUnion::Num(0),
                _ => DataUnion::None,
            },
        }
    }

    pub fn number(n: i32) -> Self {
        Self {
            ty: Ntype::Num,
            childs: None,
            value: DataUnion::Num(n),
        }
    }

    pub fn from_child(ch1: Self, ch2: Self, ty: Ntype) -> Self {
        Self {
            ty,
            childs: Some(Rc::new((ch1, ch2))),
            value: DataUnion::None,
        }
    }

    pub fn gen_tree(token: &mut TokenList) -> Result<Self, Error> {
        match Node::expr(token) {
            Ok(tr) => {
                if token.current() == token::TokenKind::Eof {
                    Ok(tr)
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "unexpected error.",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn expr(token: &mut TokenList) -> Result<Self, Error> {
        let mut node = match Node::mul(token) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };

        while token.current() != token::TokenKind::Eof {
            if token.consume(DataUnion::char('+')) {
                let mut tmp = Node::new(Ntype::Add);
                let right = match Node::mul(token) {
                    Ok(n) => n,
                    Err(e) => {
                        return Err(e);
                    }
                };
                tmp.childs = Some(Rc::new((node, right)));
                node = tmp;
            } else if token.consume(DataUnion::char('-')) {
                let mut tmp = Node::new(Ntype::Sub);
                let right = match Node::mul(token) {
                    Ok(n) => n,
                    Err(e) => {
                        return Err(e);
                    }
                };
                tmp.childs = Some(Rc::new((node, right)));
                node = tmp;
            } else {
                return Ok(node);
            }
        }
        Ok(node)
    }

    fn mul(token: &mut TokenList) -> Result<Self, Error> {
        let mut node = match Node::unary(token) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };

        while token.current() != token::TokenKind::Eof {
            if token.consume(DataUnion::char('*')) {
                let mut tmp = Node::new(Ntype::Mul);
                let right = match Node::unary(token) {
                    Ok(n) => n,
                    Err(e) => {
                        return Err(e);
                    }
                };
                tmp.childs = Some(Rc::new((node, right)));
                node = tmp;
            } else if token.consume(DataUnion::char('/')) {
                let mut tmp = Node::new(Ntype::Div);
                let right = match Node::unary(token) {
                    Ok(n) => n,
                    Err(e) => {
                        return Err(e);
                    }
                };
                tmp.childs = Some(Rc::new((node, right)));
                node = tmp;
            } else {
                return Ok(node);
            }
        }
        Ok(node)
    }

    fn unary(token: &mut TokenList) -> Result<Self, Error> {
        if token.consume(DataUnion::char('+')) {
            Node::primary(token)
        } else if token.consume(DataUnion::char('-')) {
            Ok(Node::from_child(
                Node::number(0),
                match Node::primary(token) {
                    Ok(n) => n,
                    Err(e) => {
                        return Err(e);
                    }
                },
                Ntype::Sub,
            ))
        } else {
            Node::primary(token)
        }
    }

    fn primary(token: &mut TokenList) -> Result<Self, Error> {
        if token.consume(DataUnion::char('(')) {
            let node = match Node::expr(token) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            match token.expect(DataUnion::char(')')) {
                Ok(_) => {
                    return Ok(node);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(Node::number(match token.next_number() {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        }))
    }

    fn compile(&self) -> Result<i32, Error> {
        if let Some(ch) = &self.childs {
            let n1 = match ch.0.compile() {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let n2 = match ch.1.compile() {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            Ok(match self.ty {
                Ntype::Add => n1 + n2,
                Ntype::Sub => n1 - n2,
                Ntype::Mul => n1 * n2,
                Ntype::Div => n1 / n2,
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "unexpected error."));
                }
            })
        } else if self.ty == Ntype::Num {
            if let DataUnion::Num(n) = self.value {
                Ok(n)
            } else {
                Err(Error::new(ErrorKind::InvalidData, "unexpected error."))
            }
        } else {
            Err(Error::new(ErrorKind::InvalidData, "compile error."))
        }
    }

    fn format_debug(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        if let Some(ch) = &self.childs {
            ch.0.format_debug(f, indent + 1)?;
        }
        write!(f, "{}[type:{:?}", "\t|".repeat(indent), self.ty)?;
        match self.value {
            DataUnion::None => writeln!(f, "]"),
            _ => writeln!(f, ", value:{:?}]", self.value),
        }?;
        if let Some(ch) = &self.childs {
            ch.1.format_debug(f, indent + 1)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let node = Node::new(Ntype::Add);
        println!("{:?}", node);
    }

    #[test]
    fn gen_tree_test() {
        let mut tokens = TokenList::from_file("../code.txt".to_string()).unwrap();
        println!("{:?}", tokens);
        let tree = Node::gen_tree(&mut tokens).unwrap();
        println!("{:?}", tree);
        println!("{}", tree.compile().unwrap());
    }
}
