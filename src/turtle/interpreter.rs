use regex::Regex;
use std::collections::VecDeque;
use super::token::Token;
use super::expression::Expression;

#[derive(Clone)]
pub struct Interpreter;
impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, code: &str) -> Vec<Expression> {
        let mut tokens = self.tokenize(code);
        self.interpret(&mut tokens)
    }

    fn tokenize(&self, code: &str) -> VecDeque<Token> {
        let mut tokens: VecDeque<Token> = VecDeque::new();
        let regex = Regex::new(r":*[a-zA-Z0-9]+|[0-9]+|(\[|\]|<|>|\+|-|\*)").unwrap();
        for token in regex.find_iter(code).map(|x| x.as_str()) {
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
                    match token.parse::<f32>() {
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
        tokens
    }

    fn interpret(&self, tokens: &mut VecDeque<Token>) -> Vec<Expression> {
        let mut stack: VecDeque<Token> = VecDeque::new();
        let exps = self.build(tokens, &mut stack);
        match stack.pop_back() {
            Some(Token::LBracket) => panic!("Expected closing token ']'."),
            Some(Token::To)       => panic!("Expected closing token 'end'."),
            _                     => {}
        }
        exps
    }

    fn build(&self, tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Vec<Expression> {
        let mut exps = vec!();
    
        while !tokens.is_empty() {
            let next = tokens.pop_front().unwrap();
            match next {
                Token::Forward   => exps.push(Expression::Forward(Box::new(self.build_arg(tokens)))),
                Token::Back      => exps.push(Expression::Back   (Box::new(self.build_arg(tokens)))),
                Token::Right     => exps.push(Expression::Right  (Box::new(self.build_arg(tokens)))),
                Token::Left      => exps.push(Expression::Left   (Box::new(self.build_arg(tokens)))),
                Token::Repeat    => exps.push(self.build_repeat(tokens, stack)),
                Token::If        => exps.push(self.build_if    (tokens, stack)),
                Token::To        => exps.push(self.build_to    (tokens, stack)),
                Token::Ident(x)  => exps.push(self.build_call  (tokens, x)),
                Token::RBracket  => { self.pop_stack(Token::LBracket, Token::RBracket, stack); break },
                Token::End       => { self.pop_stack(Token::To      , Token::End     , stack); break },
                _                => panic!("Unexpected token '{:?}'", next)
            };
        }
    
        exps
    }
    
    fn build_var(&self, tokens: &mut VecDeque<Token>) -> Expression {
        match tokens.pop_front() {
            Some(Token::Number(x)) => Expression::Number(x),
            Some(Token::Var(x))    => Expression::Var(x),
            Some(x)                => panic!("Unexpected token '{:?}'. Expected variable.", x),
            None                   => panic!("Expected variable, got nothing.")
        } 
    }
    
    fn build_arg(&self, tokens: &mut VecDeque<Token>) -> Expression {
        let op = tokens.get(1);
        match op {
            Some(Token::Add) => self.build_math(tokens, Box::new(Expression::Add)),
            Some(Token::Sub) => self.build_math(tokens, Box::new(Expression::Sub)),
            Some(Token::Mul) => self.build_math(tokens, Box::new(Expression::Mul)),
            Some(Token::Div) => self.build_math(tokens, Box::new(Expression::Div)),
            _                => self.build_var(tokens)
        }
    }
    
    fn build_math(&self, tokens: &mut VecDeque<Token>, op: Box<Expression>) -> Expression {
        let lhs = Box::new(self.build_var(tokens));
        tokens.pop_front();
        let rhs = Box::new(self.build_var(tokens));
    
        Expression::Math(lhs, op, rhs)
    }
    fn build_repeat(&self, tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
        let count = Box::new(self.build_arg(tokens));
        stack.push_back(Token::LBracket);
        match tokens.pop_front() {
            Some(Token::LBracket) => Expression::Repeat(count, self.build(tokens, stack)),
            Some(other)           => panic!("Unexpected token '{:?}'. Expected '['", other),
            None                  => panic!("Expected '[', got nothing.")
        }
    }
    
    fn build_if(&self, tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
        let condition = Box::new(self.build_condition(tokens));
        stack.push_back(Token::LBracket);
        match tokens.pop_front() {
            Some(Token::LBracket) => Expression::If(condition, self.build(tokens, stack)),
            Some(other)           => panic!("Unexpected token '{:?}'. Expected '['", other),
            None                  => panic!("Expected '[', got nothing.")
        }
    }
    
    fn build_condition(&self, tokens: &mut VecDeque<Token>) -> Expression {
        Expression::Condition(
            Box::new(self.build_arg(tokens)),
            Box::new(self.build_logical_op(tokens)),
            Box::new(self.build_arg(tokens))
        )
    }
    
    fn build_logical_op(&self, tokens: &mut VecDeque<Token>) -> Expression {
        match tokens.pop_front() {
            Some(Token::Less) => Expression::Less, 
            Some(Token::Gtr)  => Expression::Gtr,
            Some(other)       => panic!("Unexpected token '{:?}'. Expected logical operator.", other),
            None              => panic!("Expected logical operator, got nothing.") 
        }
    }
    
    fn build_to(&self, tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
        let ident = Box::new(self.build_name(tokens));
        stack.push_back(Token::To);
        let mut args = vec!();
        loop {
            match tokens.get(0) {
                Some(Token::Var(x)) => args.push(Expression::Var(x.to_string())),
                _                   => break
            };
            tokens.pop_front();
        }
        Expression::To(ident, args, self.build(tokens, stack))
    }
    
    fn build_name(&self, tokens: &mut VecDeque<Token>) -> Expression {
        match tokens.pop_front() {
            Some(Token::Ident(x)) => Expression::Ident(x),
            Some(x)               => panic!("Unexpected token '{:?}'. Expected identifier.", x),
            None                  => panic!("Expected identifier, got nothing.")
        }
    }
    
    fn build_call(&self, tokens: &mut VecDeque<Token>, name: String) -> Expression {
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
    
    fn pop_stack(&self, open: Token, close: Token, stack: &mut VecDeque<Token>) {
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
}