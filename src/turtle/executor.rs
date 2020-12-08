use super::expression::Expression;
use super::super::turtle::Turtle;

#[derive(Clone)]
pub struct Executor;
impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, ast: Vec<Expression>, turtle: &mut Turtle) {
        for e in ast {
            match e {
                Expression::Forward(arg)       => {
                    match *arg {
                        Expression::Number(n) => turtle.forward(n),
                        //TODO processing vars
                        _                     => {}
                    };
                },
                Expression::Repeat(count, exp) => {
                    if let Expression::Number(n) = *count {
                        for _ in 0..n as usize {
                            self.run(exp.clone(), turtle);
                        }
                    }
                },
                Expression::If(condition, exp) => {
                    if self.eval_condition(*condition) {
                        self.run(exp, turtle);
                    }
                },
                Expression::To(id, args, exp)  => {
                    turtle.add_function(id, Function::new(args, exp));
                },
                Expression::Call(id, args)     => {
                    turtle.call_function(id, args);
                },
                _                              => {}
            }
        }
    }

    fn eval_condition(&self, condition: Expression) -> bool {
        match condition {
            Expression::Condition(lhs, op, rhs) => {
                match (*op, *lhs, *rhs) {
                    (Expression::Less,
                     Expression::Number(a),
                     Expression::Number(b)) => a < b,
                     //TODO processing vars
                     _ => false
                }
            },
            _                                   => false
        }
    }
}

#[derive(Clone)]
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