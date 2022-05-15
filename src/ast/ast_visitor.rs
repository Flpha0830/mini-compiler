use crate::ast::decl::{FunDecl, Program, StructTypeDecl, VarDecl};
use crate::ast::expr::{AddressOfExpr, ArrayAccessExpr, BinOp, ChrLiteral, Expr, FieldAccessExpr, FunCallExpr, IntLiteral, Op, SizeOfExpr, StrLiteral, TypecastExpr, ValueAtExpr, VarExpr};
use crate::ast::stmt::{Assign, Block, ExprStmt, If, Return, Stmt, While};
use crate::ast::types::{ArrayType, BaseType, PointerType, StructType, Type};

pub trait ASTVisitor<T> {
    fn visit_base_type(&mut self, base_type: &mut BaseType) -> T;
    fn visit_pointer_type(&mut self, pointer_type: &mut PointerType) -> T;
    fn visit_struct_type(&mut self, struct_type: &mut StructType) -> T;
    fn visit_array_type(&mut self, array_type: &mut ArrayType) -> T;
    fn visit_program(&mut self, program: &mut Program) -> T;
    fn visit_struct_type_decl(&mut self, struct_type_decl: &mut StructTypeDecl) -> T;
    fn visit_var_decl(&mut self, var_decl: &mut VarDecl) -> T;
    fn visit_fun_decl(&mut self, fun_decl: &mut FunDecl) -> T;
    fn visit_block(&mut self, block: &mut Block) -> T;
    fn visit_while(&mut self, a_while: &mut While) -> T;
    fn visit_if(&mut self, an_if: &mut If) -> T;
    fn visit_return(&mut self, a_return: &mut Return) -> T;
    fn visit_assign(&mut self, assign: &mut Assign) -> T;
    fn visit_expr_stmt(&mut self, expr_stmt: &mut ExprStmt) -> T;
    fn visit_bin_op(&mut self, bin_op: &mut BinOp) -> T;
    fn visit_op(&mut self, op: &mut Op) -> T;
    fn visit_int_literal(&mut self, int_literal: &mut IntLiteral) -> T;
    fn visit_chr_literal(&mut self, chr_literal: &mut ChrLiteral) -> T;
    fn visit_str_literal(&mut self, str_literal: &mut StrLiteral) -> T;
    fn visit_var_expr(&mut self, var_expr: &mut VarExpr) -> T;
    fn visit_typecast_expr(&mut self, typecast_expr: &mut TypecastExpr) -> T;
    fn visit_size_of_expr(&mut self, size_of_expr: &mut SizeOfExpr) -> T;
    fn visit_address_of_exp(&mut self, address_of_expr: &mut AddressOfExpr) -> T;
    fn visit_value_at_expr(&mut self, value_at_expr: &mut ValueAtExpr) -> T;
    fn visit_field_access_exp(&mut self, field_access_expr: &mut FieldAccessExpr) -> T;
    fn visit_array_access_expr(&mut self, array_access_expr: &mut ArrayAccessExpr) -> T;
    fn visit_fun_call_expr(&mut self, fun_call_expr: &mut FunCallExpr) -> T;

    fn visit_type(&mut self, a_type: &mut Box<dyn Type>) -> T;
    fn visit_stmt(&mut self, stmt: &mut Box<dyn Stmt>) -> T;
    fn visit_expr(&mut self, expr: &mut Box<dyn Expr>) -> T;
}