#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Penup, Pendown, Setcolor,
    Forward, Back,
    Right, Left,
    Repeat, LBracket, RBracket,
    To, End,
    Number(f32), 
    Ident(String),
    Var(String),
    If, Gtr, Less, Eq, Neq,
    Add, Sub, Mul, Div
}