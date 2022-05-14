use crate::ast::ast_node::ASTNode;
use crate::ast::ast_visitor::ASTVisitor;
use crate::ast::decl::VarDecl;
use crate::ast::expr::Expr;

/// Block
pub struct Block {
    pub var_decls: Vec<VarDecl>,
    pub stmts: Vec<Box<dyn Stmt>>
}

impl Block {
    pub fn new(var_decls: Vec<VarDecl>, stmts: Vec<Box<dyn Stmt>>) -> Self {
        Block {
            var_decls,
            stmts
        }
    }
}

impl ASTNode for Block {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_block(self)
    }
}

/// While
pub struct While {
    pub expr: Box<dyn Expr>,
    pub stmt: Box<dyn Stmt>
}

impl While {
    pub fn new(expr: Box<dyn Expr>, stmt: Box<dyn Stmt>) -> Box<Self> {
        Box::new(While {
            expr,
            stmt
        })
    }
}

impl ASTNode for While {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_while(self)
    }
}

/// If
pub struct If {
    pub expr: Box<dyn Expr>,
    pub stmt1: Box<dyn Stmt>,
    pub stmt2: Box<dyn Stmt>
}

impl If {
    pub fn new(expr: Box<dyn Expr>, stmt1: Box<dyn Stmt>, stmt2: Box<dyn Stmt>) -> Box<Self> {
        Box::new(If {
            expr,
            stmt1,
            stmt2
        })
    }
}

impl ASTNode for If {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_if(self)
    }
}

/// Assign
pub struct Assign {
    pub expr1: Box<dyn Expr>,
    pub expr2: Box<dyn Expr>
}

impl Assign {
    pub fn new(expr1: Box<dyn Expr>, expr2: Box<dyn Expr>) -> Box<Self> {
        Box::new(Assign {
            expr1,
            expr2
        })
    }
}

impl ASTNode for Assign {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_assign(self)
    }
}

/// Return
pub struct Return {
    pub expr: Box<dyn Expr>
}

impl Return {
    pub fn new(expr: Box<dyn Expr>) -> Box<Self> {
        Box::new(Return {
            expr
        })
    }
}

impl ASTNode for Return {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_return(self)
    }
}

/// ExprStmt
pub struct ExprStmt {
    expr: Box<dyn Expr>,
}

impl ExprStmt {
    pub fn new(expr: Box<dyn Expr>) -> Box<Self> {
        Box::new(ExprStmt {
            expr
        })
    }
}

impl ASTNode for ExprStmt {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_expr_stmt(self)
    }
}

/// Stmt
pub trait Stmt { }

impl Stmt for Block { }
impl Stmt for While { }
impl Stmt for If { }
impl Stmt for Assign { }
impl Stmt for Return { }
impl Stmt for ExprStmt { }

