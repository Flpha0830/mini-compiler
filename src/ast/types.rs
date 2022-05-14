use crate::ast::ast_node::ASTNode;
use crate::ast::ast_visitor::ASTVisitor;

/// BaseType
pub enum BaseType {
    INT, CHAR, VOID
}

impl ASTNode for BaseType {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_base_type(self)
    }
}

/// PointerType
pub struct PointerType {
    pub pointer_type: Box<dyn Type>
}

impl PointerType {
    pub fn new(pointer_type: Box<dyn Type>) -> Box<Self> {
        Box::new(PointerType {
            pointer_type
        })
    }
}

impl ASTNode for PointerType {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_pointer_type(self)
    }
}

/// StructType
pub struct StructType {
    pub name: String
}

impl StructType {
    pub fn new(name: String) -> Self {
        StructType {
            name
        }
    }
}

impl ASTNode for StructType {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_struct_type(self)
    }
}
/// ArrayType
pub struct ArrayType {
    array_type: Box<dyn Type>,
    i: i32
}

impl ArrayType {
    pub fn new(array_type: Box<dyn Type>, i: i32) -> Self {
        ArrayType {
            array_type,
            i
        }
    }
}

impl ASTNode for ArrayType {
    fn accept<T>(&mut self, v: &mut dyn ASTVisitor<T>) -> T {
        v.visit_array_type(self)
    }
}

/// Type
pub trait Type { }

impl Type for BaseType { }
impl Type for PointerType { }
impl Type for StructType { }
impl Type for ArrayType { }