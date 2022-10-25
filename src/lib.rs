mod ast;
mod datatype;
mod error;
mod tokenize;
#[macro_use]
mod array;

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
        let mut tokens = TokenGen::from_file("./code.txt".to_string()).unwrap();
        let tree = Program::from_tokens(&mut tokens).unwrap();
        println!("{:?}", tree);
    }

    #[test]
    fn ptr_test() {
        let bo = ary![10, 20, 30];
        for e in &bo {
            println!("{}", e);
        }
    }
}
