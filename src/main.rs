use token::TokenList;
use ast::Node;
fn main() {
    let mut tokens = TokenList::from_file("./code.txt".to_string()).unwrap();
    let tree = Node::gen_tree(&mut tokens).unwrap();
    println!("{:?}", tree);
}
