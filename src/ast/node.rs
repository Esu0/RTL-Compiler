use super::rettype::RetType;
use super::Ntype;
use crate::datatype::DataUnion;
use crate::error::{Error, ErrorKind};
use crate::tokenize::{TokenGen, TokenKind};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Node {
    ty: Ntype,
    childs: Option<Rc<(Node, Node)>>,
    value: DataUnion,
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

    pub fn gen_tree(token: &mut TokenGen) -> Result<Vec<Self>, Error> {
        let tr = Node::program(token)?;
        if token.current().is_kind(TokenKind::Eof) {
            Ok(tr)
        } else {
            token.error_at("", None);
            Err(Error::new(ErrorKind::InvalidData, "unexpected error."))
        }
    }

    pub fn program(token: &mut TokenGen) -> Result<Vec<Self>, Error> {
        let mut nodes = Vec::new();
        while !token.current().is_kind(TokenKind::Eof) {
            nodes.push(Self::stmt(token)?);
        }
        Ok(nodes)
    }

    pub fn stmt(token: &mut TokenGen) -> Result<Self, Error> {
        let node = Self::expr(token)?;
        token.expect(DataUnion::char(';'))?;
        Ok(node)
    }

    fn expr(token: &mut TokenGen) -> Result<Self, Error> {
        Self::assign(token)
    }

    fn assign(token: &mut TokenGen) -> Result<Self, Error> {
        let node = Self::equality(token)?;
        if token.consume(DataUnion::char('='))? {
            Ok(Node::from_child(Self::assign(token)?, node, Ntype::Assign))
        } else {
            Ok(node)
        }
    }

    fn equality(token: &mut TokenGen) -> Result<Self, Error> {
        let mut node = Self::relational(token)?;
        loop {
            if token.consume(DataUnion::str("=="))? {
                node = Self::from_child(node, Self::relational(token)?, Ntype::Eq);
            } else if token.consume(DataUnion::str("!="))? {
                node = Self::from_child(node, Self::relational(token)?, Ntype::Neq);
            } else {
                break;
            }
        }
        Ok(node)
    }

    fn relational(token: &mut TokenGen) -> Result<Self, Error> {
        let mut node = Self::add(token)?;
        loop {
            if token.consume(DataUnion::str(">"))? {
                node = Self::from_child(node, Self::relational(token)?, Ntype::Greater);
            } else if token.consume(DataUnion::str("<"))? {
                node = Self::from_child(Self::relational(token)?, node, Ntype::Greater);
            } else if token.consume(DataUnion::str(">="))? {
                node = Self::from_child(node, Self::relational(token)?, Ntype::GreaterEq);
            } else if token.consume(DataUnion::str("<="))? {
                node = Self::from_child(Self::relational(token)?, node, Ntype::GreaterEq);
            } else {
                break;
            }
        }
        Ok(node)
    }

    fn add(token: &mut TokenGen) -> Result<Self, Error> {
        let mut node = Node::mul(token)?;

        while !token.current().is_kind(TokenKind::Eof) {
            if token.consume(DataUnion::char('+'))? {
                node = Node::from_child(node, Node::mul(token)?, Ntype::Add);
            } else if token.consume(DataUnion::char('-'))? {
                node = Node::from_child(node, Node::mul(token)?, Ntype::Sub);
            } else {
                return Ok(node);
            }
        }
        Ok(node)
    }

    fn mul(token: &mut TokenGen) -> Result<Self, Error> {
        let mut node = Node::unary(token)?;

        while !token.current().is_kind(TokenKind::Eof) {
            if token.consume(DataUnion::char('*'))? {
                node = Node::from_child(node, Node::unary(token)?, Ntype::Mul);
            } else if token.consume(DataUnion::char('/'))? {
                node = Node::from_child(node, Node::unary(token)?, Ntype::Div);
            } else {
                return Ok(node);
            }
        }
        Ok(node)
    }

    fn unary(token: &mut TokenGen) -> Result<Self, Error> {
        if token.consume(DataUnion::char('+'))? {
            Node::primary(token)
        } else if token.consume(DataUnion::char('-'))? {
            Ok(Node::from_child(
                Node::number(0),
                Node::primary(token)?,
                Ntype::Sub,
            ))
        } else {
            Node::primary(token)
        }
    }

    fn primary(token: &mut TokenGen) -> Result<Self, Error> {
        if token.consume(DataUnion::char('('))? {
            let node = Node::expr(token)?;
            token.expect(DataUnion::char(')'))?;
            Ok(node)
        } else if token.consume_kind(TokenKind::Number) {
            Ok(Node::number(token.get_number()?))
        } else {
            Ok(Node::lvar(token.get_ident()?))
        }
    }

    fn get_child(&self) -> Option<&(Self, Self)> {
        if let Some(p) = &self.childs {
            Some(&p)
        } else {
            None
        }
    }

    fn expect_child(&self) -> Result<&(Self, Self), Error> {
        if let Some(p) = &self.childs {
            Ok(&p)
        } else {
            Err(Error::new(ErrorKind::UnexpectedError, ""))
        }
    }

    pub fn compile(
        &self,
        var: &mut HashMap<Vec<char>, usize>,
        mem: &mut Vec<i32>,
    ) -> Result<RetType, Error> {
        match self.ty {
            Ntype::Add => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    ch.0.compile(var, mem)?.rvalue(mem)? + ch.1.compile(var, mem)?.rvalue(mem)?,
                ))
            }
            Ntype::Sub => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    ch.0.compile(var, mem)?.rvalue(mem)? - ch.1.compile(var, mem)?.rvalue(mem)?,
                ))
            }
            Ntype::Mul => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    ch.0.compile(var, mem)?.rvalue(mem)? * ch.1.compile(var, mem)?.rvalue(mem)?,
                ))
            }
            Ntype::Div => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    ch.0.compile(var, mem)?.rvalue(mem)? / ch.1.compile(var, mem)?.rvalue(mem)?,
                ))
            }
            Ntype::Eq => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    (ch.0.compile(var, mem)?.rvalue(mem)? == ch.1.compile(var, mem)?.rvalue(mem)?)
                        as i32,
                ))
            }
            Ntype::Neq => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    (ch.0.compile(var, mem)?.rvalue(mem)? != ch.1.compile(var, mem)?.rvalue(mem)?)
                        as i32,
                ))
            }
            Ntype::Greater => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    (ch.0.compile(var, mem)?.rvalue(mem)? > ch.1.compile(var, mem)?.rvalue(mem)?)
                        as i32,
                ))
            }
            Ntype::GreaterEq => {
                let ch = self.expect_child()?;
                Ok(RetType::Num(
                    (ch.0.compile(var, mem)?.rvalue(mem)? >= ch.1.compile(var, mem)?.rvalue(mem)?)
                        as i32,
                ))
            }
            Ntype::Assign => {
                let ch = self.expect_child()?;
                let address = ch.1.compile(var, mem)?.expect_address()?;
                mem[address] = ch.0.compile(var, mem)?.rvalue(mem)?;
                Ok(RetType::Address(address))
            }
            Ntype::Num => {
                if let DataUnion::Num(n) = self.value {
                    Ok(RetType::Num(n))
                } else {
                    Err(Error::new(ErrorKind::UnexpectedError, "unexpected error."))
                }
            }
            Ntype::Lvar => {
                if let DataUnion::String(s) = &self.value {
                    if var.contains_key(s) {
                        Ok(RetType::Address(var[s]))
                    } else {
                        let a = mem.len();
                        var.insert(s.clone(), a);
                        mem.push(0);
                        Ok(RetType::Address(a))
                    }
                } else {
                    Err(Error::new(ErrorKind::UnexpectedError, "unexpected error."))
                }
            }
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
