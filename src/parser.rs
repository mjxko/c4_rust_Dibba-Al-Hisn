// parser.rs

use crate::token::Token;
use crate::lexer::Lexer;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub current_token: Option<Token>,
    pub instructions: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            instructions: Vec::new(),
        }
    }

    pub fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_expression(&mut self, min_prec: u8) {
        // Parse primary expression
        match &self.current_token {
            Some(Token::Num(val)) => {
                self.instructions.push(format!("IMM {}", val));
                self.advance();
            }
            Some(Token::Id(name)) => {
                self.instructions.push(format!("IMM {}", name)); // placeholder
                self.advance();
            }
            Some(t) => {
                println!("Unexpected token: {:?}", t);
                self.advance();
                return;
            }
            None => return,
        }

        // Precedence climbing
        while let Some(op) = &self.current_token {
            let prec = get_precedence(op);
            if prec < min_prec {
                break;
            }

            let operator = op.clone();
            println!("Operator: {:?} (prec {})", op, prec);
            self.advance();
            self.parse_expression(prec + 1);

            match operator {
                Token::Add => println!("ADD"),
                Token::Sub => println!("SUB"),
                Token::Mul => println!("MUL"),
                Token::Div => println!("DIV"),
                Token::Mod => println!("MOD"),
                _ => println!("Unknown binary operator"),
            }
        }
    }

    pub fn parse_statement(&mut self) {
        if let Some(Token::Printf) = self.current_token {
            println!("Found printf");
            self.advance(); // move past 'printf'

            if self.current_token != Some(Token::LParen) {
                panic!("Expected '(' after printf");
            }
            self.advance(); // skip '('

            println!("Parsing expression inside printf:");
            self.parse_expression(1); // parse inside printf

            println!("PRTF"); // simulate instruction for print

            if self.current_token != Some(Token::RParen) {
                panic!("Expected ')' after printf");
            }
            self.advance(); // skip ')'

            if self.current_token != Some(Token::Semicolon) {
                panic!("Expected ';' after printf()");
            }
            self.advance(); // skip ';'
        } else if let Some(Token::Return) = self.current_token {
            println!("Found return");
            self.advance(); // move past 'return'

            self.parse_expression(1); // parse return expression

            println!("LEV"); // simulate return instruction

            if self.current_token != Some(Token::Semicolon) {
                panic!("Expected ';' after return");
            }
            self.advance(); // skip ';'
        } else {
            panic!("Unsupported statement: {:?}", self.current_token);
        }
    }
}

fn get_precedence(token: &Token) -> u8 {
    match token {
        Token::Assign => 1,
        Token::Lor => 2,
        Token::Lan => 3,
        Token::Or => 4,
        Token::Xor => 5,
        Token::And => 6,
        Token::Eq | Token::Ne => 7,
        Token::Lt | Token::Gt | Token::Le | Token::Ge => 8,
        Token::Shl | Token::Shr => 9,
        Token::Add | Token::Sub => 10,
        Token::Mul | Token::Div | Token::Mod => 11,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_printf_and_return() {
        let input = "printf(2 + 3); return 0;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        parser.parse_statement(); // should handle printf
        parser.parse_statement(); // should handle return
    }
}
