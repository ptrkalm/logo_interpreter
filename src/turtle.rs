pub mod token;
pub mod expression;
pub mod interpreter;
pub mod executor;

use interpreter::Interpreter;
use executor::Executor;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;

#[derive(Clone)]
pub struct Turtle {
    interpreter: Interpreter,
    executor:    Executor,
    image:       RgbImage,
    position:    (f32, f32)    
}

impl Turtle {
    pub fn new() -> Self {
        Self { 
            interpreter: Interpreter::new(),
            executor:    Executor::new(),
            image:       RgbImage::new(512, 512),
            position:    (256.0, 256.0)
        }
    }

    pub fn run(&mut self, code: &str) {
        let ast = self.interpreter.run(code);
        self.clone().executor.run(ast, self);
    }

    fn forward(&mut self, n: f32) {
        draw_line_segment_mut(
            &mut self.image,
            self.position,
            (self.position.0, self.position.1 - n),
            Rgb([255, 255, 255])
        );
    }
}
