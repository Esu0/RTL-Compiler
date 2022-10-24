pub mod kind;
pub use kind::ErrorKind;

#[derive(Debug)]
enum _Error {
    Simple(ErrorKind),
    Custom((ErrorKind, Box<dyn std::error::Error + Send + Sync>)),
}

pub struct Error {
    _error: _Error,
}

#[allow(dead_code)]
impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Error {
            _error: _Error::Custom((kind, error.into())),
        }
    }

    pub fn kind(&self) -> ErrorKind {
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

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self._error {
            _Error::Simple(_) => None,
            _Error::Custom(c) => c.1.source(),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            _error: _Error::Simple(kind),
        }
    }
}
