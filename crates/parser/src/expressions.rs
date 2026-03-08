pub enum Expr {
    Literal(i64),
    Add(Box<Expr>, Box<Exp>),
    Sub(Box<Expr>, Box<Exp>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Variable(String),
}
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub body: Expr,
}
