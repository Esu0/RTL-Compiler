#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub enum DataUnion {
    String(Vec<char>),
    Num(i32),
    None,
}

impl std::fmt::Debug for DataUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataUnion::None => write!(f, "None"),
            DataUnion::Num(n) => write!(f, "{n:?}"),
            DataUnion::String(s) => write!(f, "{:?}", s.iter().collect::<String>()),
        }
    }
}

impl DataUnion {
    pub fn number(n: i32) -> Self {
        Self::Num(n)
    }

    pub fn str(s: &str) -> Self {
        Self::String(s.chars().collect())
    }

    pub fn char(c: char) -> Self {
        Self::String(vec![c])
    }

    pub fn cstr(s: &[char]) -> Self {
        Self::String(s.to_vec())
    }
}
