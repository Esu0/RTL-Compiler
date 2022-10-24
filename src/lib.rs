mod ast;
mod datatype;
mod error;
mod tokenize;

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::ast::*;
    #[allow(unused_imports)]
    use super::datatype::*;
    #[allow(unused_imports)]
    use super::error::*;
    #[allow(unused_imports)]
    use super::tokenize::*;

    #[test]
    fn it_works() {
        let mut tokens = TokenList::from_file("./code.txt".to_string()).unwrap();
        println!("{:?}", tokens);
        let tree = Program::from_tokens(&mut tokens);
        println!("{:?}", tree);
    }
}
