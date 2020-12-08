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
                Expression::Forward(arg) => {
                    match *arg {
                        Expression::Number(n) => turtle.forward(n),
                        _                     => {}
                    }
                },
                _                        => {}
            }
        }
    }
}
