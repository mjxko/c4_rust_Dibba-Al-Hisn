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

// Parser
fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Keyword(k) if k == "int" => {
                if let Token::Identifier(var_name) = &tokens[i + 1] {
                    if let Token::Equal = tokens[i + 2] {
                        if let Token::Number(num) = &tokens[i + 3] {
                            let expr = Expression::Number(*num);
                            statements.push(Statement::VariableDeclaration(var_name.clone(), expr));
                            i += 5;
                        }
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

// VM: Executes the AST
fn run_vm(statements: &[Statement]) {
    let mut variables = HashMap::new();

    for stmt in statements {
        match stmt {
            Statement::VariableDeclaration(name, expr) => {
                if let Expression::Number(value) = expr {
                    variables.insert(name.clone(), *value);
                    println!("Variable '{}' set to {}", name, value);
                }
            }
            Statement::Return(expr) => {
                match expr {
                    Expression::Variable(name) => {
                        if let Some(value) = variables.get(name) {
                            println!("Return value: {}", value);
                        } else {
                            println!("Error: Undefined variable '{}'", name);
                        }
                    }
                    _ => println!("Unsupported return expression"),
                }
            }
        }
    }
}

fn main() {
    let code = "int x = 5 ; return x ;";
    let tokens = lexer(code);
    let ast = parse(&tokens);

    println!("--- VM Execution ---");
    run_vm(&ast);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let code = "int x = 5 ; return x ;";
        let tokens = lexer(code);
        assert_eq!(tokens.len(), 8);
        assert!(matches!(tokens[0], Token::Keyword(ref k) if k == "int"));
        assert!(matches!(tokens[1], Token::Identifier(_)));
        assert!(matches!(tokens[3], Token::Number(5)));
    }

    #[test]
    fn test_parser_variable_declaration() {
        let tokens = lexer("int y = 10 ;");
        let ast = parse(&tokens);
        assert_eq!(ast.len(), 1);
        if let Statement::VariableDeclaration(var, Expression::Number(val)) = &ast[0] {
            assert_eq!(var, "y");
            assert_eq!(*val, 10);
        } else {
            panic!("Parsed AST does not match expected VariableDeclaration");
        }
    }

    #[test]
    fn test_parser_return() {
        let tokens = lexer("int z = 7 ; return z ;");
        let ast = parse(&tokens);
        assert_eq!(ast.len(), 2);
        if let Statement::Return(Expression::Variable(var)) = &ast[1] {
            assert_eq!(var, "z");
        } else {
            panic!("Parsed AST does not match expected Return statement");
        }
    }
}
