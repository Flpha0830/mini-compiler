use std::any::Any;
use crate::ast::ast_visitor::ASTVisitor;

pub trait ASTNode {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T;
}

pub trait AToAny {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}