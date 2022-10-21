
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Ntype {
    Digit2,
    Digit10,
    Digit16,
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
    Operator,
    Alphabet,
    Name,
    Numalphas,
    Spname,
    String,
    Identifer,
    Switch,
    Casestmt,
    Ifstmt,
    Cond,

}

#[derive(Clone)]
#[allow(dead_code)]
struct Node {
    ty: Ntype,
    childs: Vec<Node>,
}

impl Ntype {
    fn childs_num(self) -> u8 {
        match self {
            Self::Rvalue => 3,
            Self::Lvalue => 1,
            _ => 0
        }
    }
}
impl Node {
    fn new(ty: Ntype) -> Self {
        Self {
            ty,
            childs: Vec::with_capacity(ty.childs_num() as usize)
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
