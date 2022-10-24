mod error;
mod tokenize;
mod ast;
mod datatype;

//use error::Error;
use tokenize::TokenList;
use ast::Program;

fn main() {
    let mut tokens = TokenList::from_file("./code.txt".to_string()).unwrap();
    println!("{:?}", tokens);
    let tree = Program::from_tokens(&mut tokens).unwrap();
    println!("{:?}", tree);
}
