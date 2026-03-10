use crate::expressions::Expressions;
pub enum Statements {
    VariableDecl { name: String, value: Expressions },
    Assignment { name: String, value: Expressions },
    Return(Expressions),
    Block(Vec<Statements>),
}
