use super::*;

#[derive(Clone)]
pub enum Declaration {
    ClassDecl(ClassDecl),
    FunDecl(FunDecl),
    VarDecl(VarDecl),
    Statement(Statement),
}

#[derive(Clone)]
pub struct ClassDecl {
    pub name: String,
    pub super_class: Option<String>,
    pub functions: Vec<Function>,
}

#[derive(Clone)]
pub struct FunDecl(pub Function);

#[derive(Clone)]
pub struct VarDecl {
    pub name: String,
    pub expr: Option<Expression>,
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub params: Option<Parameters>,
    pub body: Block,
}

#[derive(Clone)]
pub struct Parameters {
    pub param: String,
    pub rest: Option<Box<Parameters>>,
}

#[derive(Clone)]
pub struct Arguments {
    pub expr: Expression,
    pub rest: Option<Box<Arguments>>,
}
