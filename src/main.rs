use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
enum Token {
    Repeat,
    Forward,
    Right,
    Left,
    Number(i32),
    LBracket,
    RBracket,
}

#[derive(Debug)]
enum Expression {
    Program(Vec<Expression>),
    Repeat(i32, Vec<Expression>),
    Forward(Box<Expression>),
    Right(Box<Expression>),
    Number(i32),
    Error
}

#[derive(Debug)]
struct Repeat {
    count: u32,
    statements: Vec<Expression>
}

fn main() {
    /*let forward = Expression::Forward(10);
    let right   = Expression::Right(90);
    let repeat  = Expression::Repeat(10, vec!(forward, right));
    let program = Expression::Program(vec!(repeat)); 
    println!("{:?}", program);*/

    let mut tokens: VecDeque<Token> = VecDeque::new();
    let regex = Regex::new(r"\[|\]|[a-zA-Z]+|[0-9]*").unwrap();
    for token in regex.find_iter(SQUARE_CODE).map(|x| x.as_str()) {
        tokens.push_back(match token { 
            "repeat"  => Token::Repeat,
            "forward" => Token::Forward,
            "right"   => Token::Right,
            "left"    => Token::Left,
            "["       => Token::LBracket,
            "]"       => Token::RBracket,
            _         => Token::Number(token.parse().unwrap())
        });
    }

    println!("{:?}", tokens);
    let wtf = eval(&mut tokens,);
    println!("{:?}", wtf);
}

fn eval(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front().unwrap() {
        Token::Forward => {
            Expression::Forward(Box::new(eval(tokens)))
        },
        Token::Number(x) => {
            Expression::Number(x)
        },
        _ => Expression::Error
    }
}

const SQUARE_CODE: &str = 
"forward 50";