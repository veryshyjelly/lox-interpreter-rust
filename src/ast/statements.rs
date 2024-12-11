use super::*;

#[derive(Clone)]
pub enum Statement {
    ExprStmt(ExprStmt),
    ForStmt(ForStmt),
    IfStmt(IfStmt),
    PrntStmt(PrntStmt),
    RtrnStmt(RtrnStmt),
    WhileStmt(WhileStmt),
    Block(Block),
}

#[derive(Clone)]
pub struct ExprStmt(pub Expression);

#[derive(Clone)]
pub struct ForStmt {
    pub first_dec: ForDec,
    pub scnd_expr: Option<Expression>,
    pub thrd_expr: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Clone)]
pub enum ForDec {
    VarDecl(VarDecl),
    ExprStmt(ExprStmt),
    Nil,
}

#[derive(Clone)]
pub struct IfStmt {
    pub pred: Expression,
    pub if_stmt: Box<Statement>,
    pub else_stmt: Option<Box<Statement>>,
}

#[derive(Clone)]
pub struct PrntStmt(pub Expression);

#[derive(Clone)]
pub struct RtrnStmt(pub Option<Expression>);

#[derive(Clone)]
pub struct WhileStmt {
    pub pred: Expression,
    pub stmt: Box<Statement>,
}

#[derive(Clone)]
pub struct Block(pub Vec<Declaration>);
