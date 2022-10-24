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
        let mut tokens = TokenList::from_file("./code.txt".to_string()).unwrap();
        println!("{:?}", tokens);
        let tree = Program::from_tokens(&mut tokens);
        println!("{:?}", tree);
    }

    #[test]
    fn ptr_test() {
        let slc = &[10, 20, 30, 40];
        let ptr = slc.as_ptr();
        let bo = ary![10, 20, 30];
        let iter = slc.iter();
        unsafe {
            println!("{}", *ptr.offset(1));
        }
    }
}
