// Define all possible tokens we might find in the source code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // Literal values
    Num(i64),            // Number, like 42
    Id(String),          // Identifier, like variable or function name

    // Classes or scopes
    Fun, Sys, Glo, Loc,  // Function, System, Global, Local

    // Keywords
    Char, Else, Enum, If, Int, Return, Sizeof, While,

    // Operators and symbols
    Assign,     // =
    Cond,       // ?
    Lor, Lan,   // || and &&
    Or, Xor, And,
    Eq, Ne,     // == and !=
    Lt, Gt, Le, Ge,     // <, >, <=, >=
    Shl, Shr,   // << and >>
    Add, Sub, Mul, Div, Mod,
    Inc, Dec,   // ++ and --

    Brak,       // [
    LParen,     // (
    RParen,     // )
    Semicolon,  // ;

    // Function-like token
    Printf,

    // Any unknown or unsupported character
    Unknown(char),

    // End of input
    Eof,
}

// This enum represents the data types in our language
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Char,   // character type
    Int,    // integer type
    Ptr,    // pointer type
}

// This enum tells us the role or kind of a symbol (like a variable or function)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Num,    // constant number
    Fun,    // function
    Sys,    // system function
    Glo,    // global variable
    Loc,    // local variable
    None,   // not set
}

// Helper function to return the name of a token as a string
pub fn token_name(token: &Token) -> &'static str {
    match token {
        Token::Num(_) => "Num",               // Number
        Token::Id(_) => "Id",                 // Identifier
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

        // Operators and symbols as string versions
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
