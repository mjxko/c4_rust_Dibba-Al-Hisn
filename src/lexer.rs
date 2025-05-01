use crate::token::Token;
use std::collections::HashMap;

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current: Option<char>,
    pub keywords: HashMap<String, Token>,
    pub line: usize,
    pub col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();

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

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current {
            match c {
                ' ' | '\n' | '\r' | '\t' => {
                    self.advance();
                    continue;
                }
                '0'..='9' => {
                    let num_str = self.collect_while(|ch| ch.is_ascii_digit());
                    let val = num_str.parse::<i64>().unwrap();
                    return Some(Token::Num(val));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.collect_while(|ch| ch.is_ascii_alphanumeric() || ch == '_');
                    if let Some(tok) = self.keywords.get(&ident) {
                        return Some(tok.clone());
                    } else {
                        return Some(Token::Id(ident));
                    }
                }
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
                _ => {
                    self.advance();
                    return Some(Token::Unknown(c));
                }
            }
        }
        Some(Token::Eof)
    }
}
