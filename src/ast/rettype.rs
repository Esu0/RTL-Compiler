use crate::error::{Error, ErrorKind};

pub enum RetType {
    Num(i32),
    Address(usize),
}

impl RetType {
    pub fn expect_num(&self) -> Result<i32, Error> {
        if let Self::Num(n) = *self {
            Ok(n)
        } else {
            Err(Error::new(ErrorKind::TypeError, "expect number"))
        }
    }
    pub fn rvalue(&self, mem: &Vec<i32>) -> Result<i32, Error> {
        match self {
            Self::Num(n) => Ok(*n),
            Self::Address(a) => Ok(mem[*a]),
        }
    }
    pub fn expect_address(&self) -> Result<usize, Error> {
        if let Self::Address(a) = *self {
            Ok(a)
        } else {
            Err(Error::new(ErrorKind::TypeError, "expect address"))
        }
    }
}
