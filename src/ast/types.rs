use std::any::Any;
use std::fmt::{Display, Formatter};
use crate::ast::ast_node::{ASTNode, AToAny};
use crate::ast::ast_visitor::ASTVisitor;

/// BaseType
pub enum BaseType {
    INT, CHAR, VOID
}

impl Display for BaseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::INT => write!(f, "INT"),
            BaseType::CHAR => write!(f, "CHAR"),
            BaseType::VOID => write!(f, "VOID")
        }
    }
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
    pub array_type: Box<dyn Type>,
    pub i: i32
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
pub trait Type: AToAny { }

impl AToAny for BaseType {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Type for BaseType { }

impl AToAny for PointerType {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Type for PointerType { }

impl AToAny for StructType {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Type for StructType { }

impl AToAny for ArrayType {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Type for ArrayType { }