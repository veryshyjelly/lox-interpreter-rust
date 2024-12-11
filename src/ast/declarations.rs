use super::*;

pub enum Declaration {
    ClassDecl(ClassDecl),
    FunDecl(FunDecl),
    VarDecl(VarDecl),
    Statement(Statement),
}

pub struct ClassDecl {
    pub name: String,
    pub super_class: Option<String>,
    pub functions: Vec<Function>,
}

pub struct FunDecl(pub Function);

pub struct VarDecl {
    pub name: String,
    pub expr: Option<Expression>,
}

pub struct Function {
    pub name: String,
    pub params: Option<Parameters>,
    pub body: Block,
}

pub struct Parameters {
    pub param: String,
    pub rest: Option<Box<Parameters>>,
}

pub struct Arguments {
    pub expr: Expression,
    pub rest: Option<Box<Arguments>>,
}
