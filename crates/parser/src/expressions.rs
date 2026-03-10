pub enum Expressions {
    Literal(i64),
    Add(Box<Expressions>, Box<Expressions>),
    Sub(Box<Expressions>, Box<Expressions>),
    Mul(Box<Expressions>, Box<Expressions>),
    Div(Box<Expressions>, Box<Expressions>),
    Variable(String),
}
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub body: Expressions,
}
