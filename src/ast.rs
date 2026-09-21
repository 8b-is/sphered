#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(String),
    Sphere(Vec<Expr>),
    Txn(Vec<Step>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    Admit(Expr),
    Bind(Expr),
    Transform(Expr),
    Verify(Expr),
    Witness(Expr),
    Remember(Expr),
    Commit(Expr),
    Plain(Expr),
}
