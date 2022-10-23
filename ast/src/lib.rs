use errors::{Error, ErrorKind};
use std::rc::Rc;
use token::{DataUnion, TokenKind, TokenList};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum Ntype {
    Num,
    Lvar,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Greater,
    GreaterEq,
    Assign,
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

    pub fn lvar(name: Vec<char>) -> Self {
        Self {
            ty: Ntype::Lvar,
            childs: None,
            value: DataUnion::String(name),
        }
    }

    pub fn from_child(ch1: Self, ch2: Self, ty: Ntype) -> Self {
        Self {
            ty,
            childs: Some(Rc::new((ch1, ch2))),
            value: DataUnion::None,
        }
    }

    pub fn gen_tree(token: &mut TokenList) -> Result<Vec<Self>, Error> {
        match Node::program(token) {
            Ok(tr) => {
                if token.current() == token::TokenKind::Eof {
                    Ok(tr)
                } else {
                    token.error_at();
                    Err(Error::new(ErrorKind::InvalidData, format!("errors at {}th token", token.get_index())))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn program(token: &mut TokenList) -> Result<Vec<Self>, Error> {
        let mut nodes = Vec::new();
        while token.current() != TokenKind::Eof {
            nodes.push(match Self::stmt(token) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            });
        }
        Ok(nodes)
    }

    fn stmt(token: &mut TokenList) -> Result<Self, Error> {
        let node = match Self::expr(token) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        token.expect(DataUnion::char(';'))?;
        Ok(node)
    }

    fn expr(token: &mut TokenList) -> Result<Self, Error> {
        Self::assign(token)
    }

    fn assign(token: &mut TokenList) -> Result<Self, Error> {
        let node = match Self::equality(token) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        if token.consume(DataUnion::char('=')) {
            Ok(Node::from_child(
                node,
                match Self::assign(token) {
                    Ok(n) => n,
                    Err(e) => {
                        return Err(e);
                    }
                },
                Ntype::Assign,
            ))
        } else {
            Ok(node)
        }
    }

    fn equality(token: &mut TokenList) -> Result<Self, Error> {
        let mut node = match Self::relational(token) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        loop {
            if token.consume(DataUnion::str("==")) {
                node = Self::from_child(
                    node,
                    match Self::relational(token) {
                        Ok(n) => n,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    Ntype::Eq,
                );
            } else if token.consume(DataUnion::str("!=")) {
                node = Self::from_child(
                    node,
                    match Self::relational(token) {
                        Ok(n) => n,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    Ntype::Neq,
                );
            } else {
                break;
            }
        }
        Ok(node)
    }

    fn relational(token: &mut TokenList) -> Result<Self, Error> {
        let mut node = match Self::add(token) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        loop {
            if token.consume(DataUnion::str(">")) {
                node = Self::from_child(
                    node,
                    match Self::relational(token) {
                        Ok(n) => n,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    Ntype::Greater,
                );
            } else if token.consume(DataUnion::str("<")) {
                node = Self::from_child(
                    match Self::relational(token) {
                        Ok(n) => n,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    node,
                    Ntype::Greater,
                );
            } else if token.consume(DataUnion::str(">=")) {
                node = Self::from_child(
                    node,
                    match Self::relational(token) {
                        Ok(n) => n,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    Ntype::GreaterEq,
                );
            } else if token.consume(DataUnion::str("<=")) {
                node = Self::from_child(
                    match Self::relational(token) {
                        Ok(n) => n,
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    node,
                    Ntype::GreaterEq,
                );
            } else {
                break;
            }
        }
        Ok(node)
    }

    fn add(token: &mut TokenList) -> Result<Self, Error> {
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
        if token.consume_kind(TokenKind::Number) {
            Ok(Node::number(match token.next_number() {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            }))
        } else {
            Ok(Node::lvar(match token.next_ident() {
                Ok(s) => s,
                Err(e) => {
                    return Err(e);
                }
            }))
        }
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
        let tree = Program::from_tokens(&mut tokens).unwrap();
        println!("{:?}", tree);
        //println!("{}", tree.compile().unwrap());
    }
}
