use crate::ast::decl::{FunDecl, Program, StructTypeDecl, VarDecl};
use crate::ast::expr::{AddressOfExpr, ArrayAccessExpr, BinOp, ChrLiteral, FieldAccessExpr, FunCallExpr, IntLiteral, Op, SizeOfExpr, StrLiteral, TypecastExpr, ValueAtExpr, VarExpr};
use crate::ast::stmt::{Assign, Block, ExprStmt, If, Return, While};
use crate::ast::types::{ArrayType, BaseType, PointerType, StructType};

pub trait ASTVisitor<T> {
    fn visit_base_type(&mut self, base_type: &BaseType) -> T;
    fn visit_pointer_type(&mut self, pointer_type: &PointerType) -> T;
    fn visit_struct_type(&mut self, struct_type: &StructType) -> T;
    fn visit_array_type(&mut self, array_type: &ArrayType) -> T;
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_struct_type_decl(&mut self, struct_type_decl: &StructTypeDecl) -> T;
    fn visit_var_decl(&mut self, var_decl: &VarDecl) -> T;
    fn visit_fun_decl(&mut self, fun_decl: &FunDecl) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_while(&mut self, a_while: &While) -> T;
    fn visit_if(&mut self, an_if: &If) -> T;
    fn visit_return(&mut self, a_return: &Return) -> T;
    fn visit_assign(&mut self, assign: &Assign) -> T;
    fn visit_expr_stmt(&mut self, expr_stmt: &ExprStmt) -> T;
    fn visit_bin_op(&mut self, bin_op: &BinOp) -> T;
    fn visit_op(&mut self, op: &Op) -> T;
    fn visit_int_literal(&mut self, int_literal: &IntLiteral) -> T;
    fn visit_chr_literal(&mut self, chr_literal: &ChrLiteral) -> T;
    fn visit_str_literal(&mut self, str_literal: &StrLiteral) -> T;
    fn visit_var_expr(&mut self, var_expr: &VarExpr) -> T;
    fn visit_typecast_expr(&mut self, typecast_expr: &TypecastExpr) -> T;
    fn visit_size_of_expr(&mut self, size_of_expr: &SizeOfExpr) -> T;
    fn visit_address_of_exp(&mut self, address_of_expr: &AddressOfExpr) -> T;
    fn visit_value_at_expr(&mut self, value_at_expr: &ValueAtExpr) -> T;
    fn visit_field_access_exp(&mut self, field_access_expr: &FieldAccessExpr) -> T;
    fn visit_array_access_expr(&mut self, array_access_expr: &ArrayAccessExpr) -> T;
    fn visit_fun_call_expr(&mut self, fun_call_expr: &FunCallExpr) -> T;
}