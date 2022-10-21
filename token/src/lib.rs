
#[derive(Clone, Debug, Copy)]
#[allow(dead_code)]
enum TokenKind {
    Number,
    Reserved,
    EOF,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Token {
    tk: TokenKind,
    word: String,
}

#[allow(dead_code)]
impl Token {
    fn new(tk: TokenKind, word: String) -> Self {
        Token {
            tk,
            word,
        }
    }

    fn with_str(tk: TokenKind, word: &str) -> Self {
        Token {
            tk,
            word: String::from(word),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let token = Token::with_str(TokenKind::Number, "123");
        println!("{:?}", token);
    }
}
