use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Token {
    Repeat,
    Forward,
    Right,
    Left,
    Number(i32),
    LBracket,
    RBracket,
    To,
    Ident(String),
    End,
    Arg(String)
}

#[derive(Debug)]
enum Expression {
    Repeat(Box<Expression>, Vec<Expression>),
    To(Box<Expression>, Vec<Expression>, Vec<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Forward(Box<Expression>),
    Right(Box<Expression>),
    Number(i32),
    Ident(String),
    Arg(String)
}

fn main() {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let regex = Regex::new(r"\[|\]|:*[a-zA-Z0-9]+|[0-9]+").unwrap();
    for token in regex.find_iter(SQUARE_CODE).map(|x| x.as_str()) {
        tokens.push_back(match token { 
            "repeat"  => Token::Repeat,
            "forward" => Token::Forward,
            "right"   => Token::Right,
            "left"    => Token::Left,
            "["       => Token::LBracket,
            "]"       => Token::RBracket,
            "to"      => Token::To,
            "end"     => Token::End,
            _         => {
                match token.parse::<i32>() {
                    Ok(n) => Token::Number(n),
                    Err(_) => {
                        let string = String::from(token);
                        match string.chars().next().unwrap() {
                            ':' => Token::Arg(string),
                            _   => Token::Ident(string)
                        }
                    }
                }
            }
        });
    }

    let exps = build(&mut tokens);
    println!("{:?}", exps);
}

fn build(tokens: &mut VecDeque<Token>) -> Vec<Expression> {
    let mut exps = vec!();

    while !tokens.is_empty() {
        match tokens.pop_front().unwrap() {
            Token::Repeat    => exps.push(build_repeat(tokens)),
            Token::To        => exps.push(build_to(tokens)),
            Token::End       => break,
            Token::Forward   => exps.push(Expression::Forward(Box::new(build_arg(tokens)))),
            Token::Right     => exps.push(Expression::Right(Box::new(build_arg(tokens)))),
            Token::Number(x) => exps.push(Expression::Number(x)),
            Token::Ident(x)  => exps.push(build_call(tokens, x)),
            Token::RBracket  => break,
            _                => println!("Error")
        };
    }

    exps
}

fn build_repeat(tokens: &mut VecDeque<Token>) -> Expression {
    let count = Box::new(build_arg(tokens));
    match tokens.pop_front() {
        Some(x) => match x {
            Token::LBracket => Expression::Repeat(count, build(tokens)),
            _               => panic!("Unexpected token '{:?}'. Expected '['", x)
        },
        None    => panic!("Expected '['. Got nothing")
    }
}

fn build_to(tokens: &mut VecDeque<Token>) -> Expression {
    let ident = Box::new(build_name(tokens));
    let mut args = vec!();
    
    loop {
        match tokens.get(0) {
            Some(Token::Arg(x)) => args.push(Expression::Arg(x.to_string())),
            _                   => break
        };
        tokens.pop_front();
    }

    Expression::To(ident, args, build(tokens))
}

fn build_arg(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front() {
        Some(Token::Number(x)) => Expression::Number(x),
        Some(Token::Arg(x))    => Expression::Arg(x),
        _                      =>  panic!("Unexpected token! Expected: Number.")
    } 
}

fn build_name(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front() {
        Some(Token::Ident(x)) => Expression::Ident(x),
        _                     => panic!("Unexpected token! Expected: Indent.")
    }
}

fn build_call(tokens: &mut VecDeque<Token>, name: String) -> Expression {
    let mut args = vec!();
    
    loop {
        match tokens.get(0) {
            Some(Token::Arg(x))    => args.push(Expression::Arg(x.to_string())),
            Some(Token::Number(x)) => args.push(Expression::Number(*x)),
            _             => break
        }
        tokens.pop_front();
    }

    Expression::Call(Box::new(Expression::Ident(name)), args)
}

const SQUARE_CODE: &str = "
to rect :arg1 :arg2 
    repeat 2 [
        forward :arg1
        right 90
        forward :arg2
        right 90
    ]
end 
rect 10 20
forward 20
";