pub mod declarations;
pub mod expressions;
pub mod statements;

pub use declarations::*;
pub use expressions::*;
pub use statements::*;

pub struct Program {
    pub declarations: Vec<Declaration>,
}
