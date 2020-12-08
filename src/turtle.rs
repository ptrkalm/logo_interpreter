pub mod interpreter;
pub mod expression;
pub mod token;

use interpreter::Interpreter;

pub struct Turtle {
    interpreter: Interpreter,
    //executor:
}

impl Turtle {
    pub fn new() -> Self {
        Self { 
            interpreter: Interpreter::new()
        }
    }

    pub fn execute(&self, code: &str) {
        let ast = self.interpreter.run(code);
    }
}
