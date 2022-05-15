use std::io::{BufWriter, Stdout, Write};
use crate::ast::ast_visitor::ASTVisitor;
use crate::ast::decl::{FunDecl, Program, StructTypeDecl, VarDecl};
use crate::ast::expr::{AddressOfExpr, ArrayAccessExpr, BinOp, ChrLiteral, Expr, FieldAccessExpr, FunCallExpr, IntLiteral, Op, SizeOfExpr, StrLiteral, TypecastExpr, ValueAtExpr, VarExpr};
use crate::ast::stmt::{Assign, Block, ExprStmt, If, Return, Stmt, While};
use crate::ast::types::{ArrayType, BaseType, PointerType, StructType, Type};
use crate::ASTNode;

pub struct ASTPrinter {
    writer: BufWriter<Stdout>
}

impl ASTPrinter {
    pub fn new(writer: BufWriter<Stdout>) -> Self {
        ASTPrinter{
            writer
        }
    }
}

impl ASTVisitor<()> for ASTPrinter {
    fn visit_base_type(&mut self, base_type: &mut BaseType) -> () {
        self.writer.write(base_type.to_string().as_bytes()).unwrap();
    }

    fn visit_pointer_type(&mut self, pointer_type: &mut PointerType) -> () {
        self.writer.write(b"PointerType(").unwrap();
        self.visit_type(&mut pointer_type.pointer_type);
        self.writer.write(b")").unwrap();
    }

    fn visit_struct_type(&mut self, struct_type: &mut StructType) -> () {
        self.writer.write(b"StructType(").unwrap();
        self.writer.write(struct_type.name.as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_array_type(&mut self, array_type: &mut ArrayType) -> () {
        self.writer.write(b"ArrayType(").unwrap();
        self.visit_type(&mut array_type.array_type);
        self.writer.write(b",").unwrap();
        self.writer.write(array_type.i.to_string().as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_program(&mut self, program: &mut Program) -> () {
        self.writer.write(b"Program(").unwrap();
        let mut delimiter = String::from("");

        for struct_decl in program.struct_type_decls.iter_mut() {
            self.writer.write(delimiter.as_bytes()).unwrap();
            delimiter = ",".to_string();
            struct_decl.accept(self);
        }
        for var_decl in program.var_decls.iter_mut() {
            self.writer.write(delimiter.as_bytes()).unwrap();
            delimiter = ",".to_string();
            var_decl.accept(self);
        }
        for fun_decl in program.fun_decls.iter_mut() {
            self.writer.write(delimiter.as_bytes()).unwrap();
            delimiter = ",".to_string();
            fun_decl.accept(self);
        }
        self.writer.write(b")").unwrap();
        self.writer.flush().unwrap();
    }

    fn visit_struct_type_decl(&mut self, struct_type_decl: &mut StructTypeDecl) -> () {
        self.writer.write(b"StructTypeDecl(").unwrap();
        struct_type_decl.struct_type.accept(self);

        for var_decl in struct_type_decl.var_decls.iter_mut() {
            self.writer.write(b",").unwrap();
            var_decl.accept(self);
        }
        self.writer.write(b")").unwrap();
    }

    fn visit_var_decl(&mut self, var_decl: &mut VarDecl) -> () {
        self.writer.write(b"VarDecl(").unwrap();
        self.visit_type(&mut var_decl.var_type);
        self.writer.write(b",").unwrap();
        self.writer.write(var_decl.var_name.as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_fun_decl(&mut self, fun_decl: &mut FunDecl) -> () {
        self.writer.write(b"FunDecl(").unwrap();
        self.visit_type(&mut fun_decl.fun_type);

        self.writer.write(b",").unwrap();
        self.writer.write(fun_decl.name.as_bytes()).unwrap();
        self.writer.write(b",").unwrap();
        for var_decl in fun_decl.params.iter_mut() {
            var_decl.accept(self);
            self.writer.write(b",").unwrap();
        }
        fun_decl.block.accept(self);
        self.writer.write(b")").unwrap();
    }

    fn visit_block(&mut self, block: &mut Block) -> () {
        self.writer.write(b"Block(").unwrap();
        let mut i = 0;
        while i < block.var_decls.len() {
            if i != 0 {
                self.writer.write(b",").unwrap();
            }
            block.var_decls.get_mut(i).unwrap().accept(self);
            i += 1;
        }

        if i != 0 && block.stmts.len() != 0 {
            self.writer.write(b",").unwrap();
        }

        let mut j = 0;
        while j < block.stmts.len() {
            self.visit_stmt(&mut block.stmts.get_mut(j).unwrap());

            if j != block.stmts.len() - 1 {
                self.writer.write(b",").unwrap();
            }
            j += 1;
        }
        self.writer.write(b")").unwrap();
    }

    fn visit_while(&mut self, a_while: &mut While) -> () {
        self.writer.write(b"While(").unwrap();
        self.visit_expr(&mut a_while.expr);
        self.writer.write(b",").unwrap();
        self.visit_stmt(&mut a_while.stmt);
        self.writer.write(b")").unwrap();
    }

    fn visit_if(&mut self, an_if: &mut If) -> () {
        self.writer.write(b"If(").unwrap();
        self.visit_expr(&mut an_if.expr);
        self.writer.write(b",").unwrap();
        self.visit_stmt(&mut an_if.stmt1);
        if let Some(stmt2) = &mut an_if.stmt2 {
            self.writer.write(b",").unwrap();
            self.visit_stmt(stmt2);
        }
        self.writer.write(b")").unwrap();
    }

    fn visit_return(&mut self, a_return: &mut Return) -> () {
        self.writer.write(b"Return(").unwrap();
        if let Some(expr) = &mut a_return.expr {
            self.visit_expr(expr);
        }
        self.writer.write(b")").unwrap();
    }

    fn visit_assign(&mut self, assign: &mut Assign) -> () {
        self.writer.write(b"Assign(").unwrap();
        self.visit_expr(&mut assign.expr1);
        self.writer.write(b",").unwrap();
        self.visit_expr(&mut assign.expr2);
        self.writer.write(b")").unwrap();
    }

    fn visit_expr_stmt(&mut self, expr_stmt: &mut ExprStmt) -> () {
        self.writer.write(b"ExprStmt(").unwrap();
        self.visit_expr(&mut expr_stmt.expr);
        self.writer.write(b")").unwrap();
    }

    fn visit_bin_op(&mut self, bin_op: &mut BinOp) -> () {
        self.writer.write(b"BinOp(").unwrap();
        self.visit_expr(&mut bin_op.expr1);
        self.writer.write(b",").unwrap();
        bin_op.op.accept(self);
        self.writer.write(b",").unwrap();
        self.visit_expr(&mut bin_op.expr2);
        self.writer.write(b")").unwrap();
    }

    fn visit_op(&mut self, op: &mut Op) -> () {
        self.writer.write(op.to_string().as_bytes()).unwrap();
    }

    fn visit_int_literal(&mut self, int_literal: &mut IntLiteral) -> () {
        self.writer.write(b"IntLiteral(").unwrap();
        self.writer.write(int_literal.i.to_string().as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_chr_literal(&mut self, chr_literal: &mut ChrLiteral) -> () {
        self.writer.write(b"ChrLiteral(").unwrap();
        self.writer.write(chr_literal.c.to_string().as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_str_literal(&mut self, str_literal: &mut StrLiteral) -> () {
        self.writer.write(b"StrLiteral(").unwrap();
        self.writer.write(str_literal.string.as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_var_expr(&mut self, var_expr: &mut VarExpr) -> () {
        self.writer.write(b"VarExpr(").unwrap();
        self.writer.write(var_expr.name.as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_typecast_expr(&mut self, typecast_expr: &mut TypecastExpr) -> () {
        self.writer.write(b"TypecastExpr(").unwrap();
        self.visit_type(&mut typecast_expr.typecast_type);
        self.writer.write(b",").unwrap();
        self.visit_expr(&mut typecast_expr.expr);
        self.writer.write(b")").unwrap();
    }

    fn visit_size_of_expr(&mut self, size_of_expr: &mut SizeOfExpr) -> () {
        self.writer.write(b"SizeOfExpr(").unwrap();
        self.visit_type(&mut size_of_expr.sizeof_type);
        self.writer.write(b")").unwrap();
    }

    fn visit_address_of_exp(&mut self, address_of_expr: &mut AddressOfExpr) -> () {
        self.writer.write(b"AddressOfExp(").unwrap();
        self.visit_expr(&mut address_of_expr.expr);
        self.writer.write(b")").unwrap();
    }

    fn visit_value_at_expr(&mut self, value_at_expr: &mut ValueAtExpr) -> () {
        self.writer.write(b"ValueAtExpr(").unwrap();
        self.visit_expr(&mut value_at_expr.expr);
        self.writer.write(b")").unwrap();
    }

    fn visit_field_access_exp(&mut self, field_access_expr: &mut FieldAccessExpr) -> () {
        self.writer.write(b"FieldAccessExp(").unwrap();
        self.visit_expr(&mut field_access_expr.expr);
        self.writer.write(b",").unwrap();
        self.writer.write(field_access_expr.name.as_bytes()).unwrap();
        self.writer.write(b")").unwrap();
    }

    fn visit_array_access_expr(&mut self, array_access_expr: &mut ArrayAccessExpr) -> () {
        self.writer.write(b"ArrayAccessExpr(").unwrap();
        self.visit_expr(&mut array_access_expr.expr1);
        self.writer.write(b",").unwrap();
        self.visit_expr(&mut array_access_expr.expr2);
        self.writer.write(b")").unwrap();
    }

    fn visit_fun_call_expr(&mut self, fun_call_expr: &mut FunCallExpr) -> () {
        self.writer.write(b"FunCallExpr(").unwrap();
        self.writer.write(fun_call_expr.name.as_bytes()).unwrap();
        for expr in fun_call_expr.exprs.iter_mut() {
            self.writer.write(b",").unwrap();
            self.visit_expr(expr);
        }
        self.writer.write(b")").unwrap();
    }

    fn visit_type(&mut self, a_type: &mut Box<dyn Type>) -> () {
        if let Some(cast_type) = a_type.as_mut_any().downcast_mut::<BaseType>() {
            cast_type.accept(self);
        } else if let Some(cast_type) = a_type.as_mut_any().downcast_mut::<PointerType>() {
            cast_type.accept(self);
        } else if let Some(cast_type) = a_type.as_mut_any().downcast_mut::<StructType>() {
            cast_type.accept(self);
        } else if let Some(cast_type) = a_type.as_mut_any().downcast_mut::<ArrayType>() {
            cast_type.accept(self);
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Box<dyn Stmt>) -> () {
        if let Some(cast_stmt) = stmt.as_mut_any().downcast_mut::<Block>() {
            cast_stmt.accept(self);
        } else if let Some(cast_stmt) = stmt.as_mut_any().downcast_mut::<While>() {
            cast_stmt.accept(self);
        } else if let Some(cast_stmt) = stmt.as_mut_any().downcast_mut::<If>() {
            cast_stmt.accept(self);
        } else if let Some(cast_stmt) = stmt.as_mut_any().downcast_mut::<Assign>() {
            cast_stmt.accept(self);
        } else if let Some(cast_stmt) = stmt.as_mut_any().downcast_mut::<Return>() {
            cast_stmt.accept(self);
        } else if let Some(cast_stmt) = stmt.as_mut_any().downcast_mut::<ExprStmt>() {
            cast_stmt.accept(self);
        }
    }

    fn visit_expr(&mut self, expr: &mut Box<dyn Expr>) -> () {
        if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<IntLiteral>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<StrLiteral>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<ChrLiteral>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<VarExpr>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<FunCallExpr>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<BinOp>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<ArrayAccessExpr>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<FieldAccessExpr>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<ValueAtExpr>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<AddressOfExpr>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<SizeOfExpr>() {
            cast_expr.accept(self);
        } else if let Some(cast_expr) = expr.as_mut_any().downcast_mut::<TypecastExpr>() {
            cast_expr.accept(self);
        }
    }
}