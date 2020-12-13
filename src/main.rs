mod turtle;
use std::env;
use turtle::Turtle;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut code = String::new();
    match args.get(1) {
        Some(arg) => {
            let mut file = File::open(format!("examples/{}.logo", arg))?;
            file.read_to_string(&mut code)?;
        },
        None      => {
            code = String::from(_TEST);
        }
    };
    
    let mut turtle = Turtle::new();
    turtle.run(code);

    Ok(())
}

const _TEST: &str = "
TO RECT :A :B
    REPEAT 2 [
        FORWARD :A
        RIGHT 90
        FORWARD :B
        RIGHT 90
    ]
END
RECT 200 100
";