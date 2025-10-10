#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    Assignment = 1,     // = += -= *= /= %= &= ^= |= <<= >>=
    Conditional = 2,    // ? :
    LogicalOr = 3,      // ||
    LogicalAnd = 4,     // &&
    BitwiseOr = 5,      // |
    BitwiseXor = 6,     // ^
    BitwiseAnd = 7,     // &
    Equality = 8,       // == !=
    Relational = 9,     // < <= > >=
    Shift = 10,         // << >>
    Additive = 11,      // + -
    Multiplicative = 12, // * / %
    Unary = 13,         // ++ -- + - ! ~ * & (type)
    Postfix = 14,       // () [] . ->
    Primary = 15,       // literals, identifiers, parentheses
} 