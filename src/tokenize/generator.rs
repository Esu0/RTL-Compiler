use super::{kind::TokenKind, token::Token};
use crate::datatype::DataUnion;
use crate::error::{Error, ErrorKind};
use std::io::Read;

pub struct TokenGen {
    code: Vec<char>,
    index: usize,
    prev: usize,
    current: Token,
}

impl TokenGen {
    pub fn from_file(fp: String) -> Result<Self, Error> {
        let mut f = match std::fs::File::open(std::path::Path::new(&fp)) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(ErrorKind::CannotReadFile, e));
            }
        };
        let mut s = String::new();
        if let Err(e) = f.read_to_string(&mut s) {
            return Err(Error::new(ErrorKind::CannotReadFile, e));
        };
        let mut se = Self {
            code: s.chars().collect(),
            index: 0,
            prev: 0,
            current: Token::new(TokenKind::Eof, DataUnion::None),
        };
        se.next()?;
        Ok(se)
    }

    fn next(&mut self) -> Result<(), Error> {
        let l = self.code.len();
        let mut i = self.index;
        loop {
            if i >= l {
                self.current = Token::new(TokenKind::Eof, DataUnion::None);
                return Ok(());
            }
            if is_space(self.code[i]) {
                i += 1;
            } else {
                break;
            }
        }
        self.prev = i;
        self.index = i;
        if self.code[i] == ';' {
            i += 1;
            self.current = Token::new(TokenKind::Reserved, DataUnion::char(';'));
            self.index = i;
            Ok(())
        } else if is_ident(self.code[i]) {
            i += 1;
            while i < l && (is_ident(self.code[i]) || self.code[i].is_ascii_digit()) {
                i += 1;
            }
            self.current = Token::new(
                TokenKind::Ident,
                DataUnion::String(self.code[self.index..i].to_vec()),
            );
            self.index = i;
            Ok(())
        } else if is_mark(self.code[i]) {
            i += 1;
            while i < l && is_mark(self.code[i]) {
                i += 1;
            }
            self.current = Token::new(
                TokenKind::Reserved,
                DataUnion::String(self.code[self.index..i].to_vec()),
            );
            self.index = i;
            Ok(())
        } else if self.code[i].is_ascii_digit() {
            let mut n = self.code[i].to_digit(10).unwrap() as i32;
            i += 1;
            while i < l && self.code[i].is_ascii_digit() {
                n = n * 10 + self.code[i].to_digit(10).unwrap() as i32;
                i += 1;
            }
            self.current = Token::new(TokenKind::Number, DataUnion::Num(n));
            self.index = i;
            Ok(())
        } else {
            self.error_at("使用不可な文字が含まれています。", None);
            Err(Error::new(ErrorKind::InvalidChar, "使用不可な文字"))
        }
    }

    pub fn consume_kind(&mut self, kind: TokenKind) -> bool {
        self.current.is_kind(kind)
    }

    pub fn expect_kind(&mut self, kind: TokenKind) -> Result<(), Error> {
        if self.current.is_kind(kind) {
            self.next()
        } else {
            self.error_at("syntax error.", None);
            Err(Error::new(ErrorKind::SyntaxError, "構文エラー"))
        }
    }

    pub fn consume(&mut self, data: DataUnion) -> Result<bool, Error> {
        if self.current.eq_data(data) {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn expect(&mut self, data: DataUnion) -> Result<(), Error> {
        if self.current.eq_data(data) {
            self.next()
        } else {
            self.error_at("syntax error.", None);
            Err(Error::new(ErrorKind::SyntaxError, "構文エラー"))
        }
    }

    pub fn get_number(&mut self) -> Result<i32, Error> {
        if let DataUnion::Num(n) = self.current.get_data().clone() {
            self.next()?;
            Ok(n)
        } else {
            self.error_at(
                format!("expect Number but found {:?}", self.current.get_data()),
                None,
            );
            Err(Error::new(ErrorKind::SyntaxError, "構文エラー"))
        }
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn get_ident(&mut self) -> Result<Vec<char>, Error> {
        if self.current.is_kind(TokenKind::Ident) {
            if let DataUnion::String(s) = self.current.get_data().clone() {
                self.next()?;
                Ok(s)
            } else {
                Err(Error::new(ErrorKind::InvalidData, "unexpected error."))
            }
        } else {
            self.error_at(
                format!("expect Identity but found {:?}", self.current.get_data()),
                None,
            );
            Err(Error::new(ErrorKind::SyntaxError, "構文エラー"))
        }
    }

    pub fn error_at<S>(&self, msg: S, range: Option<(usize, usize)>)
    where
        S: std::fmt::Display,
    {
        match range {
            Some((l, r)) => {
                let pos1 = self.prev;
                let pos2 = self.index;
                println!(
                    "{} >>>{}<<< {}",
                    self.code[(std::cmp::max(pos1, l) - l)..pos1]
                        .iter()
                        .collect::<String>(),
                    self.code[pos1..pos2].iter().collect::<String>(),
                    self.code[pos2..std::cmp::min(pos2 + r, self.code.len())]
                        .iter()
                        .collect::<String>()
                );
                println!("{}", msg);
            }
            None => {
                self.error_at(msg, Some((30, 30)));
            }
        };
    }
}

fn is_space(ch: char) -> bool {
    ch.is_whitespace()
}

fn is_ident(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_mark(ch: char) -> bool {
    const MARK: &[char] = &[
        '=', '~', '|', '-', '^', '\\', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '{', '}', '[',
        ']', '`', '@', '*', ':', '/', '+', '<', '>', ',', '.', '?',
    ];
    MARK.contains(&ch)
}
