// token.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Num, Fun, Sys, Glo, Loc, Id,
    Char, Else, Enum, If, Int, Return, Sizeof, While,
    Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne, Lt, Gt,
    Le, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec, Brak,
    Unknown(char),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Char,
    Int,
    Ptr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Num,
    Fun,
    Sys,
    Glo,
    Loc,
    None,
}

pub fn token_name(token: &Token) -> &'static str {
    use Token::*;
    match token {
        Num => "Num",
        Fun => "Fun",
        Sys => "Sys",
        Glo => "Glo",
        Loc => "Loc",
        Id => "Id",
        Char => "Char",
        Else => "Else",
        Enum => "Enum",
        If => "If",
        Int => "Int",
        Return => "Return",
        Sizeof => "Sizeof",
        While => "While",
        Assign => "=",
        Cond => "?",
        Lor => "||",
        Lan => "&&",
        Or => "|",
        Xor => "^",
        And => "&",
        Eq => "==",
        Ne => "!=",
        Lt => "<",
        Gt => ">",
        Le => "<=",
        Ge => ">=",
        Shl => "<<",
        Shr => ">>",
        Add => "+",
        Sub => "-",
        Mul => "*",
        Div => "/",
        Mod => "%",
        Inc => "++",
        Dec => "--",
        Brak => "[",
        Unknown(_) => "Unknown",
        Eof => "EOF",
    }
}
