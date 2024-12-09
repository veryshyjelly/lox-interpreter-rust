use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
    ExprStmt(ExprStmt),
    ForStmt(ForStmt),
    IfStmt(IfStmt),
    PrntStmt(PrntStmt),
    RtrnStmt(RtrnStmt),
    WhileStmt(WhileStmt),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct ExprStmt(pub Expression);

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub first_dec: ForDec,
    pub scnd_expr: Option<Expression>,
    pub thrd_expr: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone)]
pub enum ForDec {
    VarDecl(VarDecl),
    ExprStmt(ExprStmt),
    Nil,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub pred: Expression,
    pub if_stmt: Box<Statement>,
    pub else_stmt: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct PrntStmt(pub Expression);

#[derive(Debug, Clone)]
pub struct RtrnStmt(pub Option<Expression>);

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub pred: Expression,
    pub stmt: Box<Statement>,
}

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Declaration>);
