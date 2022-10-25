mod ast;
mod datatype;
mod error;
mod tokenize;

//use error::Error;
use ast::Program;
use tokenize::TokenGen;

fn main() {
    let mut tokens = TokenGen::from_file("./code.txt".to_string()).unwrap();
    let tree = Program::from_tokens(&mut tokens).unwrap();
    println!("{:?}", tree);
}
