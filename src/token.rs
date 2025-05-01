// token.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Num(i64),
    Id(String),
    Fun, Sys, Glo, Loc,
    Char, Else, Enum, If, Int, Return, Sizeof, While,
    Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne, Lt, Gt,
    Le, Ge, Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec, Brak,
    LParen, RParen, Semicolon,
    Printf,
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
    match token {
        Token::Num(_) => "Num",
        Token::Id(_) => "Id",
        Token::Fun => "Fun",
        Token::Sys => "Sys",
        Token::Glo => "Glo",
        Token::Loc => "Loc",
        Token::Char => "Char",
        Token::Else => "Else",
        Token::Enum => "Enum",
        Token::If => "If",
        Token::Int => "Int",
        Token::Return => "Return",
        Token::Sizeof => "Sizeof",
        Token::While => "While",
        Token::Assign => "=",
        Token::Cond => "?",
        Token::Lor => "||",
        Token::Lan => "&&",
        Token::Or => "|",
        Token::Xor => "^",
        Token::And => "&",
        Token::Eq => "==",
        Token::Ne => "!=",
        Token::Lt => "<",
        Token::Gt => ">",
        Token::Le => "<=",
        Token::Ge => ">=",
        Token::Shl => "<<",
        Token::Shr => ">>",
        Token::Add => "+",
        Token::Sub => "-",
        Token::Mul => "*",
        Token::Div => "/",
        Token::Mod => "%",
        Token::Inc => "++",
        Token::Dec => "--",
        Token::Brak => "[",
        Token::LParen => "(",
        Token::RParen => ")",
        Token::Semicolon => ";",
        Token::Printf => "printf",
        Token::Unknown(_) => "Unknown",
        Token::Eof => "EOF",
    }
}

