use std::collections::HashMap;

/// These represent different types of tokens recognized by lexer
#[derive(Debug)]
enum Token {
    Identifier(String),Number(i32),Plus,Minus,Star,Slash,Equal,LParen,RParen,LBrace,RBrace,Semicolon, Keyword(String),
}

// This is our lexer function -> breaks code into tokens such as keywords, numbers, and so on...
fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for word in input.split_whitespace() {
        match word {
            "+" => tokens.push(Token::Plus),"-" => tokens.push(Token::Minus), // for + and -
            "*" => tokens.push(Token::Star),"/" => tokens.push(Token::Slash), // for * and /
            "=" => tokens.push(Token::Equal),";" => tokens.push(Token::Semicolon), // for = and ;
            "(" => tokens.push(Token::LParen),")" => tokens.push(Token::RParen), // for ) and ( 
            "{" => tokens.push(Token::LBrace),"}" => tokens.push(Token::RBrace), // for } and { 
            "int" | "return" => tokens.push(Token::Keyword(word.to_string())),
            num if num.chars().all(|c| c.is_digit(10)) => {
                tokens.push(Token::Number(num.parse().unwrap()))
            }
            ident => tokens.push(Token::Identifier(ident.to_string())),
        }
    }
    tokens
}

// abstract syntax tree statement types (AST)
#[derive(Debug)]
enum Statement {
    VariableDeclaration(String, Expression),
    Return(Expression),
}
// our expression types for arithmetic and variables
#[derive(Debug)]
enum Expression {
    Number(i32),
    Variable(String),
    BinaryOp(Box<Expression>, String, Box<Expression>),
}

// smarter Parser with better error handling (basically before error handling, if we write invalid code our parser would just ignore it)
fn parse(tokens: &[Token]) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Keyword(k) if k == "int" => {
                if i + 6 >= tokens.len() {
                    return Err(format!("Incomplete variable declaration at position {}", i));
                }
                if let Token::Identifier(var_name) = &tokens[i + 1] {
                    if let Token::Equal = tokens[i + 2] {
                        let expr = if let Token::Number(num1) = &tokens[i + 3] {
                            if let Token::Plus = tokens[i + 4] {
                                if let Token::Number(num2) = &tokens[i + 5] {
                                    Expression::BinaryOp(
                                        Box::new(Expression::Number(*num1)),"+".to_string(),Box::new(Expression::Number(*num2)),
                                    )
                                } else {
                                    return Err(format!("Expected number after '+' at position {}", i + 5));
                                }
                            } else {
                                Expression::Number(*num1)
                            }
                        } else {
                            return Err(format!("Expected number after '=' at position {}", i + 3));
                        };
                        statements.push(Statement::VariableDeclaration(var_name.clone(), expr));
                        i += 7;
                    } else {
                        return Err(format!("Expected '=' after the identifier at position {}", i + 2));
                    }
                } else {
                    return Err(format!("Theres an expected identifier after 'int' at position {}", i + 1));
                }
            }
            Token::Keyword(k) if k == "return" => {
                if i + 2 >= tokens.len() {
                    return Err(format!("Theres an incomplete return statement at position {}", i));
                }
                if let Token::Identifier(var_name) = &tokens[i + 1] {
                    statements.push(Statement::Return(Expression::Variable(var_name.clone())));
                    i += 3;
                } else {
                    return Err(format!("Theres an expected identifier after 'return' at position {}", i + 1));
                }
            }
            _ => {
                return Err(format!("Theres an unexpected token at position {}", i));
            }
        }
    }

    Ok(statements)
}

// Virtual Machine executor that runs the parsed AST
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

// this is an evaluator for expressions (+ and -)
fn evaluate_expression(expr: &Expression, vars: &HashMap<String, i32>) -> i32 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Variable(name) => *vars.get(name).unwrap_or(&0),
        Expression::BinaryOp(left, op, right) => {
            let l_val = evaluate_expression(left, vars);
            let r_val = evaluate_expression(right, vars);
            match op.as_str() {
                "+" => l_val + r_val,  // + expression
                "-" => l_val - r_val,  // - expression 
                _ => 0,
            }
        }
    }
}

// Main function that runs lexer, parser, and the VM
fn main() {
    let code = "int x = 5 + 3 ; return x ;";
    let tokens = lexer(code);

    match parse(&tokens) {
        Ok(ast) => {
            println!("--- VM Execution ---");
            run_vm(&ast);
        }
        Err(e) => {
            println!("Parser Error: {}", e);
        }
    }
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
        let ast = parse(&tokens).unwrap();
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
        let ast = parse(&tokens).unwrap();
    
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
