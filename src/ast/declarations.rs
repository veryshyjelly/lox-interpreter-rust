use super::*;

#[derive(Debug, Clone)]
pub enum Declaration {
    ClassDecl(ClassDecl),
    FunDecl(FunDecl),
    VarDecl(VarDecl),
    Statement(Statement),
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: String,
    pub super_class: Option<String>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct FunDecl(pub Function);

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub expr: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Option<Parameters>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub param: String,
    pub rest: Option<Box<Parameters>>,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub expr: Expression,
    pub rest: Option<Box<Arguments>>,
}
