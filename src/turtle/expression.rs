#[derive(Debug, Clone)]
pub enum Expression {
    Forward(Box<Expression>),
    Back(Box<Expression>),
    Right(Box<Expression>),
    Left(Box<Expression>),

    Repeat(Box<Expression>, Vec<Expression>),
    To(Box<Expression>, Vec<Expression>, Vec<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    
    Number(f32),
    Ident(String),
    Var(String),

    If(Box<Expression>, Vec<Expression>),
    Condition(Box<Expression>, Box<Expression>, Box<Expression>),
    
    Math(Box<Expression>, Box<Expression>, Box<Expression>),
    Less, Gtr,
    Add, Sub, Mul, Div
}