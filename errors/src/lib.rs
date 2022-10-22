use std::error;

pub struct Error {
    _error: _Error,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    InvalidChar,
    CannotReadFile,
    SyntaxError,
    InvalidData,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self 
        where
            E: Into<Box<dyn error::Error + Send + Sync>>
    {
        Error{ _error: _Error::Custom((kind, error.into()))}
    }

    pub fn kind(&self) -> ErrorKind{
        match &self._error {
            _Error::Simple(k) => *k,
            _Error::Custom(k) => k.0, 
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.kind().msg())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.kind().msg())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self._error {
            _Error::Simple(_) => None,
            _Error::Custom(c) => c.1.source(),
        }
    }
}

impl ErrorKind {
    pub fn msg(self) -> &'static str {
        match self {
            ErrorKind::InvalidChar => "使えない文字が含まれています。",
            ErrorKind::CannotReadFile => "ファイルを開けませんでした。",
            ErrorKind::SyntaxError => "構文エラー",
            ErrorKind::InvalidData => "終端トークンがデータを保持しています。"
        }
    }
}

#[derive(Debug)]
enum _Error {
    Simple(ErrorKind),
    Custom((ErrorKind, Box<dyn error::Error + Send + Sync>)),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
