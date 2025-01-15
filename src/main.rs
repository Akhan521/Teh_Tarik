use std::{env, fs};

//mod interpreter;

fn main() {
    // Retrieve the commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // Reading the file's contents.
    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let code = match result {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }
        Ok(code) => {
            code
        }
    };

    // Storing our tokens.
    let tokens = match lex(&code) {
        Err(error_msg) => {
            println!("**ERROR**");
            println!("----------------");
            println!("{}", error_msg);
            return;
        }
        Ok(tokens) => tokens
    };
    // Printing out our code and tokens.
    println!("\n\nLexing of {filename} was successful.");
    println!("-----------------------------------------------");
    println!("Code:");
    println!("{}", code);
    println!("-----------------------------------------------");
    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }
    println!("\n");

}

// Defining an enum for our tokens.
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Token {
    NotToken,
    Plus,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Assign,
    Num(i32),
    Ident(String),
    If,
    While,
    Read, 
    Func,
    Return,
    Int,
    End,
}

// This is a lexer that will take in a string of code and return a vector of tokens.
fn lex(mut code: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    while code.len() > 0 {
        // First, we'll check for numbers.
        let (is_token, token, rest_of_code) = lex_number(code);
        if is_token {
            code = rest_of_code;
            tokens.push(token);
            continue;
        }
        // Second, we'll check for spaces.
        let (is_token, rest_of_code) = lex_space(code);
        if is_token {
            code = rest_of_code;
            continue;
        }
        // Third, we'll check for plus signs.
        if code.starts_with("+") {
            code = &code[1..];
            tokens.push(Token::Plus);
            continue;
        }
        // Fourth, we'll check for minus signs.
        if code.starts_with("-") {
            code = &code[1..];
            tokens.push(Token::Subtract);
            continue;
        }
        // Fifth, we'll check for multiplication signs.
        if code.starts_with("*") {
            code = &code[1..];
            tokens.push(Token::Multiply);
            continue;
        }
        // Sixth, we'll check for division signs.
        if code.starts_with("/") {
            code = &code[1..];
            tokens.push(Token::Divide);
            continue;
        }
        // Seventh, we'll check for modulus signs.
        if code.starts_with("%") {
            code = &code[1..];
            tokens.push(Token::Modulus);
            continue;
        }
        // Eighth, we'll check for assignment signs.
        if code.starts_with("=") {
            code = &code[1..];
            tokens.push(Token::Assign);
            continue;
        }
        // Ninth, we'll check for identifiers.
        // Lastly, we'll check for invalid symbols.
        let invalid_symbol = invalid_symbol(code);
        return Err(format!("Invalid symbol: {}", invalid_symbol));
    }
    // Finally, we'll return our tokens.
    return Ok(tokens);
}

fn lex_space(code: &str) -> (bool, &str) {
    for c in code.chars() {
        if c.is_whitespace() {
            return (true, &code[1..]);
        } else {
            return (false, code);
        }
    }
    return (false, code);
}

fn lex_number(code: &str) -> (bool, Token, &str) {
    enum StateMachine {
        Start,
        Number,
    }
    // Defining our variables.
    let mut is_number = false;
    let mut state = StateMachine::Start;
    let mut index = 0;

    // Going through the characters one-by-one.
    for c in code.chars() {
        match state {
            StateMachine::Start =>{
                if c >= '0' && c <= '9' {
                    state = StateMachine::Number;
                    is_number = true;
                    index += 1;
                } else {
                    return (false, Token::NotToken, "");
                }
            }
            StateMachine::Number  => {
                if c >= '0' && c <= '9' {
                    state = StateMachine::Number;
                    is_number = true;
                    index += 1;
                } else {
                    // We have reached the end of our number token, so we'll return here.
                    let number = code[..index].parse::<i32>().unwrap();
                    // Here, we also return the rest of our string/code.
                    return (true, Token::Num(number), &code[index..]);
                }
            }
        }
    }
    // If we were successful in lexing a number, we'll return here.
    if is_number {
        let number = code[..index].parse::<i32>().unwrap();
        return (true, Token::Num(number), &code[index..]);
    } else {
        return (false, Token::NotToken, "");
    }
}

fn invalid_symbol(code: &str) -> &str {
    enum StateMachine {
        Start,
        Symbol,
    }
    // Defining our variables.
    let mut state = StateMachine::Start;
    let mut index = 0;
    // Going through the characters one-by-one.
    for c in code.chars() {
        match state {
            StateMachine::Start => {
                state = StateMachine::Symbol;
                index += 1;
            }
            StateMachine::Symbol => {
                if c.is_whitespace() {
                    return &code[..index];
                } else {
                    index += 1;
                }
            }
        }
    }
    // Returning the invalid symbol.
    return &code[..index];
}