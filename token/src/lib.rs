use std::io::Read;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd)]
#[allow(dead_code)]
pub enum TokenKind {
    Number,
    Reserved,
    Eof,
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub enum DataUnion {
    String(Vec<char>),
    Num(i32),
    None,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Token {
    tk: TokenKind,
    data: DataUnion,
}

#[allow(dead_code)]
pub struct TokenList {
    list: Vec<Token>,
    index: usize,
}

fn is_space(ch: char) -> bool {
    ch.is_whitespace()
}

fn is_reserved(ch: char) -> bool {
    ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '(' || ch == ')'
}

impl std::fmt::Debug for DataUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataUnion::None => write!(f, "None"),
            DataUnion::Num(n) => write!(f, "{n:?}"),
            DataUnion::String(s) => write!(f, "{:?}", s.iter().collect::<String>()),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, kind:{:?}]", self.data, self.tk)
    }
}

impl std::fmt::Debug for TokenList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(f, "[") {
            Err(e) => {return Err(e);},
            _ => (),
        };
        for tkn in &self.list {
            match write!(f, "{:?}, ", tkn) {
                Err(e) => {return Err(e);},
                _ => (),
            };
        }
        write!(f, "]")
    }
}

impl DataUnion {
    pub fn number(n: i32) -> Self {
        Self::Num(n)
    }

    pub fn str(s: &str) -> Self {
        Self::String(s.chars().collect())
    }

    pub fn char(c: char) -> Self {
        Self::String(vec![c])
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
}

pub fn error_at(code: &Vec<char>, pos: usize, msg: &str, range: Option<(usize, usize)>) {
    match range {
        Some((l, r)) => {
            if pos > l {
                let spaces = pos - l;
                println!(
                    "{} >>>{}",
                    code[0..spaces].iter().collect::<String>(),
                    code[spaces..(spaces + r)].iter().collect::<String>()
                );
            } else if pos + r < code.len() {
                println!(
                    "{} >>>{}",
                    code[..pos].iter().collect::<String>(),
                    code[pos..(pos + r)].iter().collect::<String>()
                );
            } else {
                println!(
                    "{} >>>{}",
                    code[..pos].iter().collect::<String>(),
                    code[pos..].iter().collect::<String>()
                );
            }
            println!("{}", msg);
        }
        None => {
            error_at(code, pos, msg, Some((30, 30)));
        }
    };
}

#[allow(dead_code)]
impl TokenList {
    pub fn gen_tokens(b: Vec<char>) -> Result<Self, std::io::Error> {
        let mut v = Vec::new();
        let mut n = 0i32;
        let mut push = false;
        for (i, elem) in b.iter().enumerate() {
            if elem.is_ascii_digit() {
                n = n * 10 + elem.to_digit(10).unwrap_or_default() as i32;
                push = true;
                continue;
            } else if push {
                v.push(Token::new(TokenKind::Number, DataUnion::Num(n)));
                n = 0;
                push = false;
            }
            if is_space(*elem) {
                continue;
            }
            if is_reserved(*elem) {
                v.push(Token::new(
                    TokenKind::Reserved,
                    DataUnion::String(vec![*elem]),
                ));
                continue;
            }
            error_at(&b, i, "構文エラー", None);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "compile error.",
            ));
        }
        if push {
            v.push(Token::new(TokenKind::Number, DataUnion::Num(n)));
        }
        v.push(Token::new(TokenKind::Eof, DataUnion::None));
        Ok(TokenList {
            list: v,
            index: 0usize,
        })
    }

    pub fn from_file(path: String) -> Result<Self, std::io::Error> {
        let mut f = match std::fs::File::open(std::path::Path::new(&path)) {
            Ok(f) => f,
            Err(e) => {
                return Err(e);
            }
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        };
        TokenList::gen_tokens(s.chars().collect())
    }
    /// If kind is matched, go to next token and it returns true.
    /// Or, it returns false.
    pub fn consume_kind(&mut self, kind: TokenKind) -> bool {
        if kind == self.list[self.index].tk {
            self.index += 1;
            return true;
        }
        false
    }

    /// if kind is matched, go to next token.
    /// Or, it returns Error.
    pub fn expect_kind(&mut self, kind: TokenKind) -> Result<(), std::io::Error> {
        if self.consume_kind(kind) {
            return Ok(());
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "different kind.",
        ))
    }

    /// If data is matched, go to next token and it returns true.
    /// Else case, returns false
    pub fn consume(&mut self, dat: DataUnion) -> bool {
        if dat == self.list[self.index].data {
            self.index += 1;
            return true;
        }
        false
    }

    pub fn expect(&mut self, dat: DataUnion) -> Result<(), std::io::Error> {
        if self.consume(dat) {
            return Ok(());
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "構文エラー",
        ))
    }

    pub fn next_number(&mut self) -> Result<i32, std::io::Error> {
        let tkn = &self.list[self.index];
        if tkn.tk == TokenKind::Number {
            self.index += 1;
            return match tkn.data {
                DataUnion::Num(n) => Ok(n),
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "unexpected error.",
                )),
            };
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "not a number.",
        ))
    }

    pub fn current(&self) -> TokenKind {
        self.list[self.index].tk
    }

    pub fn calculate(&mut self) -> Result<i32, std::io::Error> {
        let mut n = match self.next_number() {
            Ok(n) => n,
            Err(err) => {
                return Err(err);
            }
        };
        while self.current() != TokenKind::Eof {
            if self.consume(DataUnion::String(vec!['+'])) {
                n += match self.next_number() {
                    Ok(n) => n,
                    Err(err) => {
                        return Err(err);
                    }
                };
                continue;
            }
            if self.consume(DataUnion::String(vec!['-'])) {
                n -= match self.next_number() {
                    Ok(n) => n,
                    Err(err) => {
                        return Err(err);
                    }
                };
                continue;
            }
            println!("解析エラー");
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "compile error",
            ));
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    #[test]
    fn it_works() {
        let token = Token::with_str(TokenKind::Number, "123");
        println!("{:?}", token);
    }

    #[test]
    fn simple_math() {
        let path_str = "./test.txt";
        let mut f = File::open(Path::new(path_str)).expect("file open failed.");
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let mut tokens = TokenList::gen_tokens(s.chars().collect()).unwrap();
        println!("{:?}", tokens);
        let n = tokens.calculate().unwrap();
        println!("{}", n);
    }
}
