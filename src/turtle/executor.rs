use super::expression::Expression;
use super::super::turtle::Turtle;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Executor;
impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, ast: Vec<Expression>, turtle: &mut Turtle, args: &Option<HashMap<String, f32>>) {
        for e in ast {
            match e {
                Expression::Forward(arg)        => turtle.forward(self.eval_arg(arg, args)),
                Expression::Right  (arg)        => turtle.right  (self.eval_arg(arg, args)),
                Expression::Left   (arg)        => turtle.left   (self.eval_arg(arg, args)),
                Expression::Repeat (count, exp) => {
                    let n = self.eval_arg(count, args);
                    for _ in 0..n as usize {
                        self.run(exp.clone(), turtle, args);
                    }
                },
                Expression::If(condition, exp) => {
                    if self.eval_condition(*condition, args) {
                        self.run(exp, turtle, args);
                    }
                }
                Expression::To(id, args, exp)  => turtle.add_function(id, Function::new(args, exp)),
                Expression::Call(id, params)   => turtle.call_function(id, params, args),
                _                              => {}
            }
        }
    }

    fn eval_condition(&self, condition: Expression, args: &Option<HashMap<String, f32>>) -> bool {
        match condition {
            Expression::Condition(lhs, op, rhs) => {
                let a = self.eval_arg(lhs, args);
                let b = self.eval_arg(rhs, args);
                match *op {
                    Expression::Less => a < b,
                    Expression::Gtr  => a > b,
                    _ => false
                }
            },
            _                                   => false
        }
    }

    pub fn eval_arg(&self, arg: Box<Expression>, args: &Option<HashMap<String, f32>>) -> f32 {
        match *arg {
            Expression::Number(n)            => n,
            Expression::Var   (id)           => self.eval_var(id, args),
            Expression::Math  (lhs, op, rhs) => self.eval_math(lhs, op, rhs, args),
            _                                => panic!("Undefined expression occured.")
        }
    }

    fn eval_var(&self, id: String, args: &Option<HashMap<String, f32>>) -> f32 {
        match args {
            Some(map) => *map.get(&id).unwrap(),
            None      => panic!("Undefined parameter '{}'.", id)
        }
    }

    pub fn eval_math(&self, lhs: Box<Expression>, op: Box<Expression>, rhs: Box<Expression>, args: &Option<HashMap<String, f32>>) -> f32 {
        let a = self.eval_arg(lhs, args);
        let b = self.eval_arg(rhs, args);
        match *op {
            Expression::Add => a + b,
            Expression::Sub => a - b,
            Expression::Mul => a * b,
            Expression::Div => a / b,
            _               => panic!("Undefined operator occured.")
        }
    }
}


#[derive(Clone, Debug)]
pub struct Function {
    pub args: Vec<String>,
    pub exps: Vec<Expression>
}

impl Function {
    fn new(args: Vec<String>, exps: Vec<Expression>) -> Self {
        Self { 
            args,
            exps
        }
    }
}