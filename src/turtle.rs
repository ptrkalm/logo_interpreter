pub mod token;
pub mod expression;
pub mod interpreter;
pub mod executor;

use interpreter::Interpreter;
use executor::{Executor, Function};
use std::collections::HashMap;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;
use expression::Expression;

#[derive(Clone)]
pub struct Turtle {
    interpreter: Interpreter,
    executor:    Executor,
    functions:   HashMap<String, Function>,
    image:       RgbImage,
    position:    (f32, f32)
}

impl Turtle {
    pub fn new() -> Self {
        Self { 
            interpreter: Interpreter::new(),
            executor:    Executor::new(),
            functions:   HashMap::new(),
            image:       RgbImage::new(512, 512),
            position:    (256.0, 256.0)
        }
    }

    pub fn run(&mut self, code: &str) {
        let ast = self.interpreter.run(code);
        println!("{:?}", ast);
        self.clone().executor.run(ast, self, &None);
        self.image.save("output.jpg").unwrap();
    }

    fn forward(&mut self, n: f32) {
        draw_line_segment_mut(
            &mut self.image,
            self.position,
            (self.position.0, self.position.1 - n),
            Rgb([255, 255, 255])
        );
        self.position = (self.position.0, self.position.1 - n);
    }

    fn add_function(&mut self, ident: String, function: Function) {
        self.functions.insert(ident, function);
    }

    fn call_function(&mut self, ident: String, args: Vec<Expression>) {
        let function = self.functions.get(&ident).unwrap();
        let exps = function.exps.clone();
        let mut argz: HashMap<String, f32> = HashMap::new();
        for arg in args.iter().zip(function.args.clone()) {
            match arg {
                (Expression::Number(n), ident) => { argz.insert(ident, *n); },
                _ => {}
            }
        }
        self.clone().executor.run(exps, self, &Some(argz));
    }
}
