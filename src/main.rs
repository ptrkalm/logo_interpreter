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
    Forward(Box<Expression>),
    Right(Box<Expression>),
    Number(i32),
    Ident(String),
    Arg(String)
}

fn main() {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let regex = Regex::new(r"\[|\]|[:a-zA-Z0-9]+|[0-9]*").unwrap();
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

    println!("{:?}", tokens);
    let exps = eval(&mut tokens);
    println!("{:?}", exps);
}

fn eval(tokens: &mut VecDeque<Token>) -> Vec<Expression> {
    let mut exps = vec!();

    while !tokens.is_empty() {
        match tokens.pop_front().unwrap() {
            Token::Arg(x) => {
                exps.push(Expression::Arg(x))
            },
            Token::Repeat => {
                exps.push(eval_repeat(tokens));
            },
            Token::To => {
                exps.push(eval_to(tokens))
            }
            Token::End => {
                break;
            }
            Token::Forward => {
                exps.push(Expression::Forward(Box::new(eval_number(tokens))));
            },
            Token::Right => {
                exps.push(Expression::Right(Box::new(eval_number(tokens))));
            },
            Token::Number(x) => {
                exps.push(Expression::Number(x));
            },
            Token::Ident(x) => {
                exps.push(Expression::Ident(x));
            },
            Token::LBracket => {},
            Token::RBracket => { break; },
            _ => { println!("Error") }
        };
    }

    exps
}

fn eval_repeat(tokens: &mut VecDeque<Token>) -> Expression {
    Expression::Repeat(
        Box::new(eval_number(tokens)),
        eval(tokens)
    )
}

fn eval_to(tokens: &mut VecDeque<Token>) -> Expression {
    let ident = Box::new(eval_ident(tokens));
    let mut args = vec!();
    
    loop {
        let next = tokens.pop_front().unwrap();
        match next {
            Token::Arg(x) => args.push(Expression::Arg(x)),
            _             => break
        };
    }

    Expression::To(ident, args, eval(tokens))
}

fn eval_number(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front().unwrap() {
        Token::Number(x) => {
            Expression::Number(x)
        },
        Token::Arg(x) => {
            Expression::Arg(x)
        }
        _ => {
            panic!("Unexpected token! Expected: Number.");
        }
    } 
}

fn eval_ident(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front().unwrap() {
        Token::Ident(x) => {
            Expression::Ident(x)
        },
        _ => {
            panic!("Unexpected token! Expected: Indent.")
        }
    }
}

const SQUARE_CODE: &str = 
"to rect :arg1 :arg2 repeat 2 [forward :arg1 right 90 forward :arg2 right 90] end";