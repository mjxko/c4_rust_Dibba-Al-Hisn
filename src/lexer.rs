use crate::token::Token;
use std::collections::HashMap;

// This is the structure of our lexer. It reads the input code one character at a time.
pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,         // The iterator over characters in the input
    current: Option<char>,              // The current character we are looking at
    pub keywords: HashMap<String, Token>, // A list of reserved words like 'if', 'return', etc.
    pub line: usize,                    // Keeps track of the current line number (for debugging)
    pub col: usize,                     // Keeps track of the current column
}

impl<'a> Lexer<'a> {
    // This function creates a new Lexer with the input string
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();       // Convert the input to characters
        let current = chars.next();          // Get the first character

        // Define the reserved words and match them to tokens
        let mut keywords = HashMap::new();
        keywords.insert("if".to_string(), Token::If);
        keywords.insert("else".to_string(), Token::Else);
        keywords.insert("int".to_string(), Token::Int);
        keywords.insert("char".to_string(), Token::Char);
        keywords.insert("return".to_string(), Token::Return);
        keywords.insert("while".to_string(), Token::While);
        keywords.insert("sizeof".to_string(), Token::Sizeof);
        keywords.insert("printf".to_string(), Token::Printf);

        Lexer {
            chars,
            current,
            keywords,
            line: 1,
            col: 0,
        }
    }

    // Moves to the next character and updates line/column counters
    fn advance(&mut self) {
        if let Some(c) = self.current {
            if c == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
        self.current = self.chars.next();
    }

    // Collects characters as long as they match the given condition
    fn collect_while<F>(&mut self, mut condition: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.current {
            if condition(c) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    // This is the core function that returns the next token (e.g. number, keyword, operator)
    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current {
            match c {
                // Skip whitespace and move to the next character
                ' ' | '\n' | '\r' | '\t' => {
                    self.advance();
                    continue;
                }

                // If it's a digit, collect all digits and return a Num token
                '0'..='9' => {
                    let num_str = self.collect_while(|ch| ch.is_ascii_digit());
                    let val = num_str.parse::<i64>().unwrap();
                    return Some(Token::Num(val));
                }

                // If it's a letter or _, it's an identifier or a keyword
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.collect_while(|ch| ch.is_ascii_alphanumeric() || ch == '_');
                    if let Some(tok) = self.keywords.get(&ident) {
                        return Some(tok.clone()); // it's a keyword like 'if' or 'return'
                    } else {
                        return Some(Token::Id(ident)); // it's a user-defined name
                    }
                }

                // Operators and symbols
                '=' => {
                    self.advance();
                    if self.current == Some('=') {
                        self.advance();
                        return Some(Token::Eq);
                    }
                    return Some(Token::Assign);
                }
                '!' => {
                    self.advance();
                    if self.current == Some('=') {
                        self.advance();
                        return Some(Token::Ne);
                    }
                    return Some(Token::Unknown('!'));
                }
                '<' => {
                    self.advance();
                    if self.current == Some('=') {
                        self.advance();
                        return Some(Token::Le);
                    } else if self.current == Some('<') {
                        self.advance();
                        return Some(Token::Shl);
                    }
                    return Some(Token::Lt);
                }
                '>' => {
                    self.advance();
                    if self.current == Some('=') {
                        self.advance();
                        return Some(Token::Ge);
                    } else if self.current == Some('>') {
                        self.advance();
                        return Some(Token::Shr);
                    }
                    return Some(Token::Gt);
                }
                '|' => {
                    self.advance();
                    if self.current == Some('|') {
                        self.advance();
                        return Some(Token::Lor);
                    }
                    return Some(Token::Or);
                }
                '&' => {
                    self.advance();
                    if self.current == Some('&') {
                        self.advance();
                        return Some(Token::Lan);
                    }
                    return Some(Token::And);
                }
                '+' => {
                    self.advance();
                    if self.current == Some('+') {
                        self.advance();
                        return Some(Token::Inc);
                    }
                    return Some(Token::Add);
                }
                '-' => {
                    self.advance();
                    if self.current == Some('-') {
                        self.advance();
                        return Some(Token::Dec);
                    }
                    return Some(Token::Sub);
                }
                '*' => {
                    self.advance();
                    return Some(Token::Mul);
                }
                '/' => {
                    self.advance();
                    return Some(Token::Div);
                }
                '%' => {
                    self.advance();
                    return Some(Token::Mod);
                }
                '(' => {
                    self.advance();
                    return Some(Token::LParen);
                }
                ')' => {
                    self.advance();
                    return Some(Token::RParen);
                }
                ';' => {
                    self.advance();
                    return Some(Token::Semicolon);
                }

                // If it's something we don't recognize, return it as Unknown
                _ => {
                    self.advance();
                    return Some(Token::Unknown(c));
                }
            }
        }

        // End of input
        Some(Token::Eof)
    }
}
