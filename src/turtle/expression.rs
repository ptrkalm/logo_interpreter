#[derive(Debug, Clone)]
pub enum Expression {
    Penup, Pendown,
    Setcolor(Box<Expression>, Box<Expression>, Box<Expression>),

    Forward (Box<Expression>),
    Back    (Box<Expression>),
    Right   (Box<Expression>),
    Left    (Box<Expression>),

    Repeat(Box<Expression>, Vec<Expression>),
    To    (String, Vec<String>, Vec<Expression>),
    Call  (String, Vec<Expression>),
    
    Number(f32),
    Var   (String),

    If       (Box<Expression>, Vec<Expression>),
    Condition(Box<Expression>, Box<Expression>, Box<Expression>),
    Less, Gtr, Eq, Neq,

    Math(Box<Expression>, Box<Expression>, Box<Expression>),
    Add, Sub, Mul, Div
}