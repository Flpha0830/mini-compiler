use crate::ast::ast_visitor::ASTVisitor;

pub trait ASTNode {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T;
}