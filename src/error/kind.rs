#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    InvalidChar,
    CannotReadFile,
    SyntaxError,
    InvalidData,
}

impl ErrorKind {
    pub fn msg(self) -> &'static str {
        match self {
            ErrorKind::InvalidChar => "使えない文字が含まれています。",
            ErrorKind::CannotReadFile => "ファイルを開けませんでした。",
            ErrorKind::SyntaxError => "構文エラー",
            ErrorKind::InvalidData => "終端トークンがデータを保持しています。",
        }
    }
}
