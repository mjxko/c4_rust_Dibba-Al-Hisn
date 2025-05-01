use crate::token::Token;
use crate::lexer::Lexer;

// The parser reads tokens from the lexer and turns them into instructions
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,              // Where we get tokens from
    pub current_token: Option<Token>,  // The current token we're looking at
    pub instructions: Vec<String>,     // The list of instructions we will generate
}

impl<'a> Parser<'a> {
    // Make a new parser and get the first token ready
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            instructions: Vec::new(),
        }
    }

    // Move to the next token
    pub fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    // Start parsing the whole program (loop through all statements)
    pub fn parse_program(&mut self) {
        while self.current_token != Some(Token::Eof) {
            self.parse_statement(); // parse one statement at a time
        }
    }

    // Parse an expression like "2 + 3" or "x * y"
    pub fn parse_expression(&mut self, min_prec: u8) {
        // First, handle numbers or variables
        match &self.current_token {
            Some(Token::Num(val)) => {
                self.instructions.push(format!("IMM {}", val)); // Push the number to instructions
                self.advance(); // Go to next token
            }
            Some(Token::Id(name)) => {
                self.instructions.push(format!("IMM {}", name)); // Placeholder for variables
                self.advance();
            }
            Some(t) => {
                println!("Unexpected token: {:?}", t); // If it's something weird
                self.advance();
                return;
            }
            None => return, // No more tokens
        }

        // Handle operators like +, -, *, etc. based on precedence
        while let Some(op) = &self.current_token {
            let prec = get_precedence(op);
            if prec < min_prec {
                break;
            }

            let operator = op.clone(); // Save the operator
            println!("Operator: {:?} (prec {})", op, prec);
            self.advance(); // Move past operator
            self.parse_expression(prec + 1); // Recursively parse next part

            // Print the kind of operation we just handled
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

    // Handle full statements like printf(...) or return ...
    pub fn parse_statement(&mut self) {
        if let Some(Token::Printf) = self.current_token {
            println!("Found printf");
            self.advance(); // Move past 'printf'

            if self.current_token != Some(Token::LParen) {
                panic!("Expected '(' after printf");
            }
            self.advance(); // Skip '('

            println!("Parsing expression inside printf:");
            self.parse_expression(1); // Get the value to print

            println!("PRTF"); // Simulate a print instruction

            if self.current_token != Some(Token::RParen) {
                panic!("Expected ')' after printf");
            }
            self.advance(); // Skip ')'

            if self.current_token != Some(Token::Semicolon) {
                panic!("Expected ';' after printf()");
            }
            self.advance(); // Skip ';'
        } else if let Some(Token::Return) = self.current_token {
            println!("Found return");
            self.advance(); // Move past 'return'

            self.parse_expression(1); // Get the value to return

            println!("LEV"); // Simulate function return

            if self.current_token != Some(Token::Semicolon) {
                panic!("Expected ';' after return");
            }
            self.advance(); // Skip ';'
        } else {
            // We don’t support other statements yet
            panic!("Unsupported statement: {:?}", self.current_token);
        }
    }
}

// This gives each operator a priority (higher number = stronger)
// For example, * and / come before + and -
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

    #[test]
    #[should_panic(expected = "Syntax Error")]
    fn test_error_reporting_missing_paren() {
        let input = "printf(2 + 3;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_statement(); // should panic here
    }
}


