pub enum Expr {
    Literal(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Variable(String),
}
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub body: Expr,
}
