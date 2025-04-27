use std::collections::HashMap;

#[derive(Debug)]
enum Token {
    Identifier(String),
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Keyword(String),
}

// Lexer
fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for word in input.split_whitespace() {
        match word {
            "+" => tokens.push(Token::Plus),
            "-" => tokens.push(Token::Minus),
            "*" => tokens.push(Token::Star),
            "/" => tokens.push(Token::Slash),
            "=" => tokens.push(Token::Equal),
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            "{" => tokens.push(Token::LBrace),
            "}" => tokens.push(Token::RBrace),
            ";" => tokens.push(Token::Semicolon),
            "int" | "return" => tokens.push(Token::Keyword(word.to_string())),
            num if num.chars().all(|c| c.is_digit(10)) => {
                tokens.push(Token::Number(num.parse().unwrap()))
            }
            ident => tokens.push(Token::Identifier(ident.to_string())),
        }
    }

    tokens
}

// AST
#[derive(Debug)]
enum Statement {
    VariableDeclaration(String, Expression),
    Return(Expression),
}

#[derive(Debug)]
enum Expression {
    Number(i32),
    Variable(String),
    BinaryOp(Box<Expression>, String, Box<Expression>),
}

// Smarter Parser
fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Keyword(k) if k == "int" => {
                if let Token::Identifier(var_name) = &tokens[i + 1] {
                    if let Token::Equal = tokens[i + 2] {
                        let expr = if let Token::Number(num1) = &tokens[i + 3] {
                            if let Token::Plus = tokens[i + 4] {
                                if let Token::Number(num2) = &tokens[i + 5] {
                                    Expression::BinaryOp(
                                        Box::new(Expression::Number(*num1)),
                                        "+".to_string(),
                                        Box::new(Expression::Number(*num2)),
                                    )
                                } else {
                                    Expression::Number(*num1)
                                }
                            } else {
                                Expression::Number(*num1)
                            }
                        } else {
                            Expression::Number(0)
                        };
                        statements.push(Statement::VariableDeclaration(var_name.clone(), expr));
                        i += 7;
                    }
                }
            }
            Token::Keyword(k) if k == "return" => {
                if let Token::Identifier(var_name) = &tokens[i + 1] {
                    statements.push(Statement::Return(Expression::Variable(var_name.clone())));
                    i += 3;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    statements
}

// VM + Expression Evaluator
fn run_vm(statements: &[Statement]) {
    let mut variables = HashMap::new();

    for stmt in statements {
        match stmt {
            Statement::VariableDeclaration(name, expr) => {
                let value = evaluate_expression(expr, &variables);
                variables.insert(name.clone(), value);
                println!("Variable '{}' set to {}", name, value);
            }
            Statement::Return(expr) => {
                let value = evaluate_expression(expr, &variables);
                println!("Return value: {}", value);
            }
        }
    }
}

fn evaluate_expression(expr: &Expression, vars: &HashMap<String, i32>) -> i32 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Variable(name) => *vars.get(name).unwrap_or(&0),
        Expression::BinaryOp(left, op, right) => {
            let l_val = evaluate_expression(left, vars);
            let r_val = evaluate_expression(right, vars);
            match op.as_str() {
                "+" => l_val + r_val,
                "-" => l_val - r_val,
                _ => 0,
            }
        }
    }
}

// Main function
fn main() {
    let code = "int x = 5 + 3 ; return x ;";
    let tokens = lexer(code);
    let ast = parse(&tokens);

    println!("--- VM Execution ---");
    run_vm(&ast);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_arithmetic() {
        let code = "int x = 5 + 3 ; return x ;";
        let tokens = lexer(code);
        assert_eq!(tokens.len(), 10);
        assert!(matches!(tokens[0], Token::Keyword(ref k) if k == "int"));
        assert!(matches!(tokens[3], Token::Number(5)));
        assert!(matches!(tokens[5], Token::Number(3)));
    }

    #[test]
    fn test_parser_binary_op() {
        let tokens = lexer("int x = 5 + 3 ;");
        let ast = parse(&tokens);
        assert_eq!(ast.len(), 1);
        if let Statement::VariableDeclaration(var, expr) = &ast[0] {
            assert_eq!(var, "x");
            if let Expression::BinaryOp(_, op, _) = expr {
                assert_eq!(op, "+");
            } else {
                panic!("Expected BinaryOp in expression");
            }
        } else {
            panic!("Expected VariableDeclaration");
        }
    }

    #[test]
    fn test_vm_execution() {
        let tokens = lexer("int x = 5 + 3 ; return x ;");
        let ast = parse(&tokens);
    
        let mut vars = HashMap::new();
        for stmt in &ast {
            if let Statement::VariableDeclaration(name, expr) = stmt {
                let value = super::evaluate_expression(expr, &vars);
                assert_eq!(value, 8);
                vars.insert(name.clone(), value);
            }
            if let Statement::Return(expr) = stmt {
                let value = super::evaluate_expression(expr, &vars);
                assert_eq!(value, 8);
            }
        }
    }
    
}
