use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        Lexer { input, chars, current }
    }

    fn advance(&mut self) {
        self.current = self.chars.next();
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current {
            match c {
                // doesnt count whitespaces
                ' ' | '\n' | '\r' | '\t' => {
                    self.advance();
                    continue;
                }
                // any number
                '0'..='9' => {
                    self.advance();
                    return Some(Token::Num);
                }
                // any identifier 
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.advance();
                    return Some(Token::Id);
                }
                '+' => { // for addition
                    self.advance();
                    return Some(Token::Add);
                }
                '-' => { // for subtraction
                    self.advance();
                    return Some(Token::Sub);
                }
                '*' => { // for multiplication
                    self.advance();
                    return Some(Token::Mul);
                }
                '/' => { // for division
                    self.advance();
                    return Some(Token::Div);
                }
                '=' => { // for equal sign
                    self.advance();
                    return Some(Token::Assign);
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
