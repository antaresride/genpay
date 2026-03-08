pub enum Stmt {
    VariableDecl { name: String, value: Expr },
    Assignment { name: String, value: Expr },
    Return(Expr),
    Block(Vec<Stmt>),
}
