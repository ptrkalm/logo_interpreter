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
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Turtle {
    interpreter: Interpreter,
    executor:    Executor,
    functions:   HashMap<String, Function>,
    image:       RgbImage,
    position:    (f32, f32),
    angle:       f32
}

impl Turtle {
    pub fn new() -> Self {
        Self { 
            interpreter: Interpreter::new(),
            executor:    Executor::new(),
            functions:   HashMap::new(),
            image:       RgbImage::new(512, 512),
            position:    (256.0, 256.0),
            angle:       0.0
        }
    }

    pub fn run(&mut self, code: &str) {
        let ast = self.interpreter.run(code);
        println!("{:?}", ast);
        self.clone().executor.run(ast, self, &None);
        self.image.save("output.jpg").unwrap();
    }

    fn forward(&mut self, n: f32) {
        let x = self.position.0 + (self.angle * PI / 180.0).sin() * n;
        let y = self.position.1 - (self.angle * PI / 180.0).cos() * n;
        draw_line_segment_mut(
            &mut self.image,
            self.position,
            (x, y),
            Rgb([255, 255, 255])
        );
        self.position = (x, y);
    }

    fn right(&mut self, n: f32) {
        self.angle = (((self.angle + n).floor() as i32) % 360) as f32 + self.angle.fract();
    }

    fn left(&mut self, n: f32) {
        self.angle = (((self.angle - n).floor() as i32) % 360) as f32 + self.angle.fract();
    }

    fn add_function(&mut self, ident: String, function: Function) {
        self.functions.insert(ident, function);
    }

    fn call_function(&mut self, ident: String, params: Vec<Expression>, args: &Option<HashMap<String, f32>>) {
        let function = self.functions.get(&ident).unwrap();
        let exps = function.exps.clone();
        let mut argz: HashMap<String, f32> = HashMap::new();
        for arg in params.iter().zip(function.args.clone()) {
            match arg {
                (exp, ident) => {
                    let n = self.executor.eval_arg(Box::new(exp.clone()), args);
                    argz.insert(ident, n);
                }
                //_ => {}
            }
        }
        self.clone().executor.run(exps, self, &Some(argz));
    }
}
