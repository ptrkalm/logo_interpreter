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

    pub fn run(&self, code: String) -> Vec<Expression> {
        let mut tokens = self.tokenize(code.to_lowercase().as_str());
        self.interpret(&mut tokens)
    }

    fn tokenize(&self, code: &str) -> VecDeque<Token> {
        let mut tokens: VecDeque<Token> = VecDeque::new();
        let regex = Regex::new(r":*[a-zA-Z]+[0-9]?+|-?\d+(\.\d+)?|(\[|\]|!=|==|<|>|\+|-|\*|/)").unwrap();
        for token in regex.find_iter(code).map(|x| x.as_str()) {
            tokens.push_back(match token {
                "penup"    | "pu" => Token::Penup,
                "pendown"  | "pd" => Token::Pendown,
                "setcolor" | "sc" => Token::Setcolor,
                "forward"  | "fd" => Token::Forward,
                "back"     | "bk" => Token::Back,
                "right"    | "rt" => Token::Right,
                "left"     | "lt" => Token::Left,
                "repeat"   | "rp" => Token::Repeat,
                "["        => Token::LBracket,
                "]"        => Token::RBracket,
                "to"       => Token::To,
                "end"      => Token::End,
                "if"       => Token::If,
                ">"        => Token::Gtr,
                "<"        => Token::Less,
                "=="       => Token::Eq,
                "!="       => Token::Neq,
                "+"        => Token::Add,
                "-"        => Token::Sub,
                "*"        => Token::Mul,
                "/"        => Token::Div,
                _          => {
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
                Token::Penup     => exps.push(Expression::Penup),
                Token::Pendown   => exps.push(Expression::Pendown),
                Token::Setcolor  => exps.push(self.build_set_color(tokens)),
                Token::Forward   => exps.push(Expression::Forward (Box::new(self.build_arg(tokens).unwrap()))),
                Token::Back      => exps.push(Expression::Back    (Box::new(self.build_arg(tokens).unwrap()))),
                Token::Right     => exps.push(Expression::Right   (Box::new(self.build_arg(tokens).unwrap()))),
                Token::Left      => exps.push(Expression::Left    (Box::new(self.build_arg(tokens).unwrap()))),
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

    fn build_set_color(&self, tokens: &mut VecDeque<Token>) -> Expression {
        let r = Box::new(self.build_arg(tokens).unwrap());
        let g = Box::new(self.build_arg(tokens).unwrap());
        let b = Box::new(self.build_arg(tokens).unwrap());
        Expression::Setcolor(r, g, b)
    }
    
    fn build_var(&self, tokens: &mut VecDeque<Token>) -> Option<Expression> {
        match tokens.pop_front() {
            Some(Token::Number(x)) => Some(Expression::Number(x)),
            Some(Token::Var(x))    => Some(Expression::Var(x)),
            _                      => None 
        } 
    }
    
    fn build_arg(&self, tokens: &mut VecDeque<Token>) -> Option<Expression> {
        let op = tokens.get(1);
        match op {
            Some(Token::Add) => self.build_math(tokens, Box::new(Expression::Add)),
            Some(Token::Sub) => self.build_math(tokens, Box::new(Expression::Sub)),
            Some(Token::Mul) => self.build_math(tokens, Box::new(Expression::Mul)),
            Some(Token::Div) => self.build_math(tokens, Box::new(Expression::Div)),
            Some(_) | None   => self.build_var(tokens),
        }
    }
    
    fn build_math(&self, tokens: &mut VecDeque<Token>, op: Box<Expression>) -> Option<Expression> {
        let lhs = Box::new(self.build_var(tokens).unwrap());
        tokens.pop_front();
        let rhs = Box::new(self.build_var(tokens).unwrap());
    
        Some(Expression::Math(lhs, op, rhs))
    }
    fn build_repeat(&self, tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
        let count = Box::new(self.build_arg(tokens).unwrap());
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
            Box::new(self.build_arg(tokens).unwrap()),
            Box::new(self.build_logical_op(tokens)),
            Box::new(self.build_arg(tokens).unwrap())
        )
    }
    
    fn build_logical_op(&self, tokens: &mut VecDeque<Token>) -> Expression {
        match tokens.pop_front() {
            Some(Token::Less) => Expression::Less, 
            Some(Token::Gtr)  => Expression::Gtr,
            Some(Token::Eq)   => Expression::Eq,
            Some(Token::Neq)  => Expression::Neq,
            Some(other)       => panic!("Unexpected token '{:?}'. Expected logical operator.", other),
            None              => panic!("Expected logical operator, got nothing.") 
        }
    }
    
    fn build_to(&self, tokens: &mut VecDeque<Token>, stack: &mut VecDeque<Token>) -> Expression {
        let ident = self.build_name(tokens);
        stack.push_back(Token::To);
        let mut args = vec!();
        loop {
            match tokens.get(0) {
                Some(Token::Var(x)) => args.push(x.to_string()),
                _                   => break
            };
            tokens.pop_front();
        }
        Expression::To(ident, args, self.build(tokens, stack))
    }
    
    fn build_name(&self, tokens: &mut VecDeque<Token>) -> String {
        match tokens.pop_front() {
            Some(Token::Ident(x)) => x,
            Some(x)               => panic!("Unexpected token '{:?}'. Expected identifier.", x),
            None                  => panic!("Expected identifier, got nothing.")
        }
    }
    
    fn build_call(&self, tokens: &mut VecDeque<Token>, name: String) -> Expression {
        let mut args = vec!();
        
        loop {
            match tokens.get(0) {
                Some(Token::Var(_)) | Some(Token::Number(_)) => {
                    args.push(self.build_arg(tokens).unwrap())
                },
                _ => break
            };
        }
    
        Expression::Call(name, args)
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