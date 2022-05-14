use crate::ast::ast_node::ASTNode;
use crate::ast::ast_visitor::ASTVisitor;
use crate::ast::decl::{FunDecl, VarDecl};
use crate::ast::stmt::Block;
use crate::ast::types::{BaseType, Type};

/// IntLiteral
pub struct IntLiteral {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    i: i32
}

impl IntLiteral {
    pub fn new(i: i32) -> Self {
        IntLiteral {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            i
        }
    }
}

impl ASTNode for IntLiteral {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_int_literal(self)
    }
}

/// StrLiteral
pub struct StrLiteral {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    string: String
}

impl StrLiteral {
    pub fn new(string: String) -> Self {
        StrLiteral {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            string
        }
    }
}

impl ASTNode for StrLiteral {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_str_literal(self)
    }
}

/// ChrLiteral
pub struct ChrLiteral {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    c: char
}

impl ChrLiteral {
    pub fn new(data: String) -> Self {
        ChrLiteral {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            c: data.chars().next().unwrap()
        }
    }
}

impl ASTNode for ChrLiteral {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_chr_literal(self)
    }
}

/// VarExpr
pub struct VarExpr {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    name: String,
    var_decl: VarDecl // to be filled in by the name analyser
}

impl VarExpr {
    pub fn new(name: String) -> Self {
        VarExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            name,
            var_decl: VarDecl::new(Box::new(BaseType::VOID), "".to_string())
        }
    }
}

impl ASTNode for VarExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_var_expr(self)
    }
}

/// FunCallExpr
pub struct FunCallExpr {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    name: String,
    exprs: Vec<Box<dyn Expr>>,
    fun_decl: FunDecl
}

impl FunCallExpr {
    pub fn new(name: String, exprs: Vec<Box<dyn Expr>>) -> Self {
        FunCallExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            name,
            exprs,
            fun_decl: FunDecl::new(Box::new(BaseType::VOID), "".to_string(), vec![], Block { var_decls: vec![], stmts: vec![] })
        }
    }
}

impl ASTNode for FunCallExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_fun_call_expr(self)
    }
}

/// BinOp
pub struct BinOp {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    op: Op,
    expr1: Box<dyn Expr>,
    expr2: Box<dyn Expr>
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
pub enum Op {
    ADD , SUB , MUL , DIV , MOD , GT , LT , GE , LE , NE , EQ , OR , AND
}

impl ASTNode for Op{
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_op(self)
    }
}

/// ArrayAccessExpr
pub struct ArrayAccessExpr {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    expr1: Box<dyn Expr>,
    expr2: Box<dyn Expr>
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
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    expr: Box<dyn Expr>,
    name: String
}

impl FieldAccessExpr {
    pub fn new(expr: Box<dyn Expr>, name: String) -> Self {
        FieldAccessExpr {
            expr_type: Box::new(BaseType::VOID),
            is_grouped: false,
            expr,
            name
        }
    }
}

impl ASTNode for FieldAccessExpr {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_field_access_exp(self)
    }
}

/// ValueAtExpr
pub struct ValueAtExpr {
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    expr: Box<dyn Expr>,
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
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    expr: Box<dyn Expr>,
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
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    sizeof_type: Box<dyn Type>,
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
    expr_type: Box<dyn Type>,
    is_grouped: bool,
    typecast_type: Box<dyn Type>,
    expr: Box<dyn Expr>,
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

pub trait Expr { }

impl Expr for IntLiteral { }
impl Expr for StrLiteral { }
impl Expr for ChrLiteral { }
impl Expr for VarExpr { }
impl Expr for FunCallExpr { }
impl Expr for BinOp { }
impl Expr for ArrayAccessExpr { }
impl Expr for FieldAccessExpr { }
impl Expr for ValueAtExpr { }
impl Expr for AddressOfExpr { }
impl Expr for SizeOfExpr { }
impl Expr for TypecastExpr { }
