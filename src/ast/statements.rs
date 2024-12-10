use super::*;

pub enum Statement {
    ExprStmt(ExprStmt),
    ForStmt(ForStmt),
    IfStmt(IfStmt),
    PrntStmt(PrntStmt),
    RtrnStmt(RtrnStmt),
    WhileStmt(WhileStmt),
    Block(Block),
}

pub struct ExprStmt(pub Expression);

pub struct ForStmt {
    pub first_dec: ForDec,
    pub scnd_expr: Option<Expression>,
    pub thrd_expr: Option<Expression>,
    pub body: Box<Statement>,
}

pub enum ForDec {
    VarDecl(VarDecl),
    ExprStmt(ExprStmt),
    Nil,
}

pub struct IfStmt {
    pub pred: Expression,
    pub if_stmt: Box<Statement>,
    pub else_stmt: Option<Box<Statement>>,
}

pub struct PrntStmt(pub Expression);

pub struct RtrnStmt(pub Option<Expression>);

pub struct WhileStmt {
    pub pred: Expression,
    pub stmt: Box<Statement>,
}

pub struct Block(pub Vec<Declaration>);
