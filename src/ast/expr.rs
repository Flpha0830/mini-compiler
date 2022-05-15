use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::ast::ast_node::{ASTNode, AToAny};
use crate::ast::ast_visitor::ASTVisitor;
use crate::ast::decl::{FunDecl, VarDecl};
use crate::ast::stmt::Block;
use crate::ast::types::{BaseType, Type};

/// IntLiteral
pub struct IntLiteral {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub i: i32
}

impl IntLiteral {
    pub fn new(data: String) -> Box<Self> {
        Box::new(IntLiteral {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            i: data.parse::<i32>().unwrap()
        })
    }
}

impl ASTNode for IntLiteral {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_int_literal(self)
    }
}

/// StrLiteral
pub struct StrLiteral {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub string: String
}

impl StrLiteral {
    pub fn new(string: String) -> Box<Self> {
        Box::new(StrLiteral {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            string
        })
    }
}

impl ASTNode for StrLiteral {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_str_literal(self)
    }
}

/// ChrLiteral
pub struct ChrLiteral {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub c: char
}

impl ChrLiteral {
    pub fn new(data: String) -> Box<Self> {
        Box::new(ChrLiteral {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            c: data.chars().next().unwrap()
        })
    }
}

impl ASTNode for ChrLiteral {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_chr_literal(self)
    }
}

/// VarExpr
pub struct VarExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub name: String,
    pub var_decl: VarDecl // to be filled in by the name analyser
}

impl VarExpr {
    pub fn new(name: String) -> Box<Self> {
        Box::new(VarExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            name,
            var_decl: VarDecl::new(Box::new(BaseType::VOID), "".to_string())
        })
    }
}

impl ASTNode for VarExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_var_expr(self)
    }
}

/// FunCallExpr
pub struct FunCallExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub name: String,
    pub exprs: Vec<Box<dyn Expr>>,
    pub fun_decl: FunDecl
}

impl FunCallExpr {
    pub fn new(name: String, exprs: Vec<Box<dyn Expr>>) -> Box<Self> {
        Box::new(FunCallExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            name,
            exprs,
            fun_decl: FunDecl::new(Box::new(BaseType::VOID), "".to_string(), vec![], Block { var_decls: vec![], stmts: vec![] })
        })
    }
}

impl ASTNode for FunCallExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_fun_call_expr(self)
    }
}

/// BinOp
pub struct BinOp {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub op: Op,
    pub expr1: Box<dyn Expr>,
    pub expr2: Box<dyn Expr>
}

impl BinOp {
    pub fn new(expr1: Box<dyn Expr>, op: Op, expr2: Box<dyn Expr>) -> Box<Self> {
        Box::new(BinOp {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            op,
            expr1,
            expr2
        })
    }
}

impl ASTNode for BinOp {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_bin_op(self)
    }
}

/// Op
#[derive(Clone, Copy)]
pub enum Op {
    ADD , SUB , MUL , DIV , MOD , GT , LT , GE , LE , NE , EQ , OR , AND
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::ADD => write!(f, "ADD"),
            Op::SUB => write!(f, "SUB"),
            Op::MUL => write!(f, "MUL"),
            Op::DIV => write!(f, "DIV"),
            Op::MOD => write!(f, "MOD"),
            Op::GT => write!(f, "GT"),
            Op::GE => write!(f, "GE"),
            Op::LT => write!(f, "LT"),
            Op::LE => write!(f, "LE"),
            Op::NE => write!(f, "NE"),
            Op::EQ => write!(f, "EQ"),
            Op::OR => write!(f, "OR"),
            Op::AND => write!(f, "AND"),
        }
    }
}

impl ASTNode for Op{
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_op(self)
    }
}

/// ArrayAccessExpr
pub struct ArrayAccessExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub expr1: Box<dyn Expr>,
    pub expr2: Box<dyn Expr>
}

impl ArrayAccessExpr {
    pub fn new(expr1: Box<dyn Expr>, expr2: Box<dyn Expr>) -> Box<Self> {
        Box::new(ArrayAccessExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            expr1,
            expr2
        })
    }
}

impl ASTNode for ArrayAccessExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_array_access_expr(self)
    }
}

/// FieldAccessExpr
pub struct FieldAccessExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub expr: Box<dyn Expr>,
    pub name: String
}

impl FieldAccessExpr {
    pub fn new(expr: Box<dyn Expr>, name: String) -> Box<Self> {
        Box::new(FieldAccessExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            expr,
            name
        })
    }
}

impl ASTNode for FieldAccessExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_field_access_exp(self)
    }
}

/// ValueAtExpr
pub struct ValueAtExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub expr: Box<dyn Expr>,
}

impl ValueAtExpr {
    pub fn new(expr: Box<dyn Expr>) -> Box<Self> {
        Box::new(ValueAtExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            expr
        })
    }
}

impl ASTNode for ValueAtExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_value_at_expr(self)
    }
}

/// AddressOfExpr
pub struct AddressOfExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub expr: Box<dyn Expr>,
}

impl AddressOfExpr {
    pub fn new(expr: Box<dyn Expr>) -> Box<Self> {
        Box::new(AddressOfExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            expr
        })
    }
}

impl ASTNode for AddressOfExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_address_of_exp(self)
    }
}

/// SizeOfExpr
pub struct SizeOfExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub sizeof_type: Box<dyn Type>,
}

impl SizeOfExpr {
    pub fn new(sizeof_type: Box<dyn Type>) -> Box<Self> {
        Box::new(SizeOfExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            sizeof_type
        })
    }
}

impl ASTNode for SizeOfExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_size_of_expr(self)
    }
}

/// TypecastExpr
pub struct TypecastExpr {
    pub expr_type: Box<dyn Type>,
    pub is_grouped: bool,
    pub typecast_type: Box<dyn Type>,
    pub expr: Box<dyn Expr>,
}

impl TypecastExpr {
    pub fn new(typecast_type: Box<dyn Type>, expr: Box<dyn Expr>) -> Box<Self> {
        Box::new(TypecastExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            typecast_type,
            expr
        })
    }
}

impl ASTNode for TypecastExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_typecast_expr(self)
    }
}

/// Expr
pub trait Expr: AToAny {
    fn get_is_grouped(&self) -> bool;
    fn set_is_grouped(&mut self, is_grouped: bool);
}

impl AToAny for IntLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for IntLiteral {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for StrLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for StrLiteral {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for ChrLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for ChrLiteral {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for VarExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for VarExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for FunCallExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for FunCallExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for BinOp {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for BinOp {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for ArrayAccessExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for ArrayAccessExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for FieldAccessExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for FieldAccessExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for ValueAtExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for ValueAtExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for AddressOfExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for AddressOfExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for SizeOfExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for SizeOfExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}

impl AToAny for TypecastExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expr for TypecastExpr {
    fn get_is_grouped(&self) -> bool {
        self.is_grouped
    }

    fn set_is_grouped(&mut self, is_grouped: bool) {
        self.is_grouped = is_grouped
    }
}
