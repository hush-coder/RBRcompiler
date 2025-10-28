use std::collections::HashSet;

pub fn c_keywords() -> HashSet<&'static str> {
    [
        "int",
        "void",
        "if",
        "else",
        "return",
    ]
    .into_iter()
    .collect()
} 