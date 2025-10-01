use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CType {
    Int,
    Float,
    Char,
    Void,
}

impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl CType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "int" => Some(CType::Int),
            "float" => Some(CType::Float),
            "char" => Some(CType::Char),
            "void" => Some(CType::Void),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CType::Int => "int",
            CType::Float => "float",
            CType::Char => "char",
            CType::Void => "void",
        }
    }
}