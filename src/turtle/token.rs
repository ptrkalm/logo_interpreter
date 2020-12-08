#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Forward, Back, Right, Left,
    Repeat, LBracket, RBracket,
    To, End,
    Number(f32), 
    Ident(String),
    Var(String),
    If, Gtr, Less,
    Add, Sub, Mul, Div
}