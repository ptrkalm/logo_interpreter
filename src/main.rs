use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Token {
    Forward, Back, Right,Left,
    Repeat, LBracket, RBracket,
    To, End,
    Number(i32), Ident(String), Var(String),
    If, Gtr, Less,
    Add, Sub, Mul, Div
}

#[derive(Debug)]
enum Expression {
    Forward(Box<Expression>),
    Back(Box<Expression>),
    Right(Box<Expression>),
    Left(Box<Expression>),

    Repeat(Box<Expression>, Vec<Expression>),
    To(Box<Expression>, Vec<Expression>, Vec<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    
    Number(i32),
    Ident(String),
    Var(String),

    If(Box<Expression>, Vec<Expression>),
    Condition(Box<Expression>, Box<Expression>, Box<Expression>),
    
    Math(Box<Expression>, Box<Expression>, Box<Expression>),
    Less, Gtr,
    Add, Sub, Mul, Div
}

fn main() {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let regex = Regex::new(r":*[a-zA-Z0-9]+|[0-9]+|(\[|\]|<|>|\+|-|\*)").unwrap();
    for token in regex.find_iter(SQUARE_CODE).map(|x| x.as_str()) {
        tokens.push_back(match token { 
            "forward" => Token::Forward,
            "back"    => Token::Back,
            "right"   => Token::Right,
            "left"    => Token::Left,
            "repeat"  => Token::Repeat,
            "["       => Token::LBracket,
            "]"       => Token::RBracket,
            "to"      => Token::To,
            "end"     => Token::End,
            "if"      => Token::If,
            ">"       => Token::Gtr,
            "<"       => Token::Less,
            "+"       => Token::Add,
            "-"       => Token::Sub,
            "*"       => Token::Mul,
            "/"       => Token::Div,
            _         => {
                match token.parse::<i32>() {
                    Ok(n) => Token::Number(n),
                    Err(_) => {
                        let string = String::from(token);
                        match string.chars().next().unwrap() {
                            ':' => Token::Var(string),
                            _   => Token::Ident(string)
                        }
                    }
                }
            }
        });
    }

    let mut stack: VecDeque<Token> = VecDeque::new();
    let exps = build(&mut tokens, &mut stack);
    match stack.pop_back() {
        Some(Token::LBracket) => panic!("Expected closing token ']'."),
        Some(Token::To)       => panic!("Expected closing token 'end'."),
        _                     => {}
    }
    println!("{:?}", exps);
    println!("{:?}", stack);
}

fn build(tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Vec<Expression> {
    let mut exps = vec!();

    while !tokens.is_empty() {
        let next = tokens.pop_front().unwrap();
        match next {
            Token::Forward   => exps.push(Expression::Forward(Box::new(build_arg(tokens)))),
            Token::Back      => exps.push(Expression::Back   (Box::new(build_arg(tokens)))),
            Token::Right     => exps.push(Expression::Right  (Box::new(build_arg(tokens)))),
            Token::Left      => exps.push(Expression::Left   (Box::new(build_arg(tokens)))),
            Token::Repeat    => exps.push(build_repeat(tokens, stack)),
            Token::If        => exps.push(build_if    (tokens, stack)),
            Token::To        => exps.push(build_to    (tokens, stack)),
            Token::Ident(x)  => exps.push(build_call  (tokens, x)),
            Token::RBracket  => { pop_stack(Token::LBracket, Token::RBracket, stack); break },
            Token::End       => { pop_stack(Token::To      , Token::End     , stack); break },
            _                => panic!("Unexpected token '{:?}'", next)
        };
    }

    exps
}

fn build_var(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front() {
        Some(Token::Number(x)) => Expression::Number(x),
        Some(Token::Var(x))    => Expression::Var(x),
        Some(x)                => panic!("Unexpected token '{:?}'. Expected variable.", x),
        None                   => panic!("Expected variable, got nothing.")
    } 
}

fn build_arg(tokens: &mut VecDeque<Token>) -> Expression {
    let op = tokens.get(1);
    match op {
        Some(Token::Add) => build_math(tokens, Box::new(Expression::Add)),
        Some(Token::Sub) => build_math(tokens, Box::new(Expression::Sub)),
        Some(Token::Mul) => build_math(tokens, Box::new(Expression::Mul)),
        Some(Token::Div) => build_math(tokens, Box::new(Expression::Div)),
        _                => build_var(tokens)
    }
}

fn build_math(tokens: &mut VecDeque<Token>, op: Box<Expression>) -> Expression {
    let lhs = Box::new(build_var(tokens));
    tokens.pop_front();
    let rhs = Box::new(build_var(tokens));

    Expression::Math(lhs, op, rhs)
}
fn build_repeat(tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
    let count = Box::new(build_arg(tokens));
    stack.push_back(Token::LBracket);
    match tokens.pop_front() {
        Some(Token::LBracket) => Expression::Repeat(count, build(tokens, stack)),
        Some(other)           => panic!("Unexpected token '{:?}'. Expected '['", other),
        None                  => panic!("Expected '[', got nothing.")
    }
}

fn build_if(tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
    let condition = Box::new(build_condition(tokens));
    stack.push_back(Token::LBracket);
    match tokens.pop_front() {
        Some(Token::LBracket) => Expression::If(condition, build(tokens, stack)),
        Some(other)           => panic!("Unexpected token '{:?}'. Expected '['", other),
        None                  => panic!("Expected '[', got nothing.")
    }
}

fn build_condition(tokens: &mut VecDeque<Token>) -> Expression {
    Expression::Condition(
        Box::new(build_arg(tokens)),
        Box::new(build_logical_op(tokens)),
        Box::new(build_arg(tokens))
    )
}

fn build_logical_op(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front() {
        Some(Token::Less) => Expression::Less, 
        Some(Token::Gtr)  => Expression::Gtr,
        Some(other)       => panic!("Unexpected token '{:?}'. Expected logical operator.", other),
        None              => panic!("Expected logical operator, got nothing.") 
    }
}

fn build_to(tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
    let ident = Box::new(build_name(tokens));
    stack.push_back(Token::To);
    let mut args = vec!();
    loop {
        match tokens.get(0) {
            Some(Token::Var(x)) => args.push(Expression::Var(x.to_string())),
            _                   => break
        };
        tokens.pop_front();
    }
    Expression::To(ident, args, build(tokens, stack))
}

fn build_name(tokens: &mut VecDeque<Token>) -> Expression {
    match tokens.pop_front() {
        Some(Token::Ident(x)) => Expression::Ident(x),
        Some(x)               => panic!("Unexpected token '{:?}'. Expected identifier.", x),
        None                  => panic!("Expected identifier, got nothing.")
    }
}

fn build_call(tokens: &mut VecDeque<Token>, name: String) -> Expression {
    let mut args = vec!();
    
    loop {
        match tokens.get(0) {
            Some(Token::Var(x))    => args.push(Expression::Var(x.to_string())),
            Some(Token::Number(x)) => args.push(Expression::Number(*x)),
            _                      => break
        }
        tokens.pop_front();
    }

    Expression::Call(Box::new(Expression::Ident(name)), args)
}

fn pop_stack(open: Token, close: Token, stack: &mut VecDeque<Token>) {
    match stack.pop_back() {
        Some(token)  => {
            match token == open {
                true  => {},
                false => panic!("Expected opening token '{:?}' before '{:?}'.", open, close)
            }
        },
        None         => panic!("Expected opening token '{:?}' before '{:?}'.", open, close)
    }
}

const SQUARE_CODE: &str ="
to rect :arg1 :arg2
    if :arg1 * 2 > :arg2 [
        repeat 2 [
            forward :arg1
            right 90
            forward :arg2
            right 90
        ]
    ]
end
rect 10 20
";

/*
[
    To(
        Ident("rect"),
        [Var(":arg1"), Var(":arg2")],
        [
            If(
                Condition(
                    Math(Var(":arg1"), Mul, Number(2)),
                    Gtr,
                    Var(":arg2")
                ),
                [
                    Repeat(
                        Number(2),
                        [
                            Forward(Var(":arg1")),
                            Right(Number(90)),
                            Forward(Var(":arg2")),
                            Right(Number(90))
                        ]
                    )
                ]
            )
        ]
    ),
    Call(Ident("rect"), [Number(10), Number(20)])]
*/