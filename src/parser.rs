// parser.rs

use crate::token::Token;
use crate::lexer::Lexer;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    pub fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_expression(&mut self, min_prec: u8) {
        // Parse primary expression
        let mut lhs = match &self.current_token {
            Some(Token::Num) => {
                println!("Num");
                self.advance();
                "Num"
            }
            Some(Token::Id) => {
                println!("Id");
                self.advance();
                "Id"
            }
            Some(t) => {
                println!("Unexpected token: {:?}", t);
                self.advance();
                return;
            }
            None => return,
        };

        // Precedence climbing
        while let Some(op) = &self.current_token {
            let prec = get_precedence(op);
            if prec < min_prec {
                break;
            }

            println!("Operator: {:?} (prec {})", op, prec);
            self.advance();
            self.parse_expression(prec + 1);
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
    fn test_parse_expression_with_addition() {
        let input = "a + b * c";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_expression(1);
    }
}
