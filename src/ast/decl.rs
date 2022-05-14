use crate::ast::ast_node::ASTNode;
use crate::ast::ast_visitor::ASTVisitor;
use crate::ast::stmt::Block;
use crate::ast::types::{StructType, Type};

/// Program
pub struct Program {
    pub struct_type_decls: Vec<StructTypeDecl>,
    pub var_decls: Vec<VarDecl>,
    pub fun_decls: Vec<FunDecl>,
}

impl Program {
    pub fn new(struct_type_decls: Vec<StructTypeDecl>, var_decls: Vec<VarDecl>, fun_decls: Vec<FunDecl>) -> Self {
        Program {
            struct_type_decls,
            var_decls,
            fun_decls
        }
    }
}

impl ASTNode for Program {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_program(self)
    }
}

/// StructTypeDecl
pub struct StructTypeDecl {
    pub struct_type: StructType,
    pub var_decls: Vec<VarDecl>,
}

impl StructTypeDecl {
    pub fn new(struct_type: StructType, var_decls: Vec<VarDecl>) -> Self {
        StructTypeDecl{
            struct_type,
            var_decls
        }
    }
}

impl ASTNode for StructTypeDecl {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_struct_type_decl(self)
    }
}

/// VarDecl
pub struct VarDecl {
    pub var_type: Box<dyn Type>,
    pub var_name: String,
}

impl VarDecl {
    pub fn new(var_type: Box<dyn Type>, var_name: String) -> Self {
        VarDecl {
            var_type,
            var_name
        }
    }
}

impl ASTNode for VarDecl {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_var_decl(self)
    }
}

/// FunDecl
pub struct FunDecl {
    pub fun_type: Box<dyn Type>,
    pub name: String,
    pub params: Vec<VarDecl>,
    pub block: Block,
}

impl FunDecl {
    pub fn new(fun_type: Box<dyn Type>, name: String, params: Vec<VarDecl>, block: Block) -> Self {
        FunDecl{
            fun_type,
            name,
            params,
            block
        }
    }
}

impl ASTNode for FunDecl {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_fun_decl(self)
    }
}