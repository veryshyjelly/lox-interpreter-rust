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
    first_dec: ForDec,
    scnd_expr: Option<Expression>,
    thrd_expr: Option<Expression>,
    statement: Box<Statement>,
}

#[derive(Debug, Clone)]
pub enum ForDec {
    VarDecl(VarDecl),
    ExprStmt(ExprStmt),
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pred: Expression,
    if_stmt: Box<Statement>,
    else_stmt: Option<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub struct PrntStmt(pub Expression);

#[derive(Debug, Clone)]
pub struct RtrnStmt(Option<Expression>);
#[derive(Debug, Clone)]
pub struct WhileStmt {
    pred: Expression,
    stmt: Box<Statement>,
}
#[derive(Debug, Clone)]
pub struct Block {
    declarations: Vec<Declaration>,
}
