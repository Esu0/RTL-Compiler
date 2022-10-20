
#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Ntype {
    Digit,
    Num,
    State,
    Memory,
    Register,
    Stmt,
    Stmtset,
    RTop,
    RTops,
    RTL,
    Lvalue,
    Rvalue,
    Operand,
    Operator,
}

#[derive(Clone)]
#[allow(dead_code)]
struct Node {
    ty: Ntype,
    childs: Vec<Node>,
}

impl Node {
    fn new(ty: Ntype) -> Self {
        Self {
            ty,
            childs: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let node = Node::new(Ntype::Operator);
        println!("{:?}", node.ty);
    }
}
