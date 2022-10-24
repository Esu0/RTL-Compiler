#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum Ntype {
    Num,
    Lvar,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Greater,
    GreaterEq,
    Assign,
}

#[allow(dead_code)]
impl Ntype {
    fn has_childs(self) -> bool {
        self > Ntype::Num
    }
}
