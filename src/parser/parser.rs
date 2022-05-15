use std::collections::VecDeque;
use std::rc::Rc;
use crate::lexer::token::Token;
use crate::{TokenClass, Tokenizer};
use crate::ast::decl::{FunDecl, Program, StructTypeDecl, VarDecl};
use crate::ast::expr::{AddressOfExpr, ArrayAccessExpr, BinOp, ChrLiteral, Expr, FieldAccessExpr, FunCallExpr, IntLiteral, Op, SizeOfExpr, StrLiteral, TypecastExpr, ValueAtExpr, VarExpr};
use crate::ast::stmt::{Assign, Block, ExprStmt, If, Return, Stmt, While};
use crate::ast::types::{ArrayType, BaseType, PointerType, StructType, Type};


macro_rules! return_if {
    ( $e:expr; $list:ident ) => {
        if $e { return $list; }
    }
}

pub struct Parser {
    token: Rc<Token>,
    // use for backtracking (useful for distinguishing decls from procs when parsing a program for instance)
    buffer: VecDeque<Token>,
    tokenizer: Tokenizer,
    error: i32,
    last_error_token: Rc<Token>,
}

impl Parser {
    pub fn new(mut tokenizer: Tokenizer) -> Self {
        Parser {
            token: Rc::new(tokenizer.next_token()),
            buffer: VecDeque::new(),
            tokenizer,
            error: 0,
            last_error_token: Rc::new(Token::new(TokenClass::INVALID, "", 0, 0))
        }
    }

    pub fn get_error_count(&self) -> i32 {
        self.error
    }

    fn error(&mut self, expected: &[TokenClass]) {
        if Rc::ptr_eq(&self.last_error_token, &self.token) {
            // skip this error, same token causing trouble
            return;
        }

        let mut tokens = String::new();
        let mut sep = "";

        for e in expected {
            tokens.push_str(sep);
            tokens.push_str(e.to_string().as_str());
            sep = "|";
        }

        println!("Parsing error: expected ({}) found ({}) at {}",tokens, self.token, self.token.position);

        self.error += 1;
        self.last_error_token = Rc::clone(&self.token);
    }

    /*
     * Look ahead the i^th element from the stream of token.
     * i should be >= 1
     */
    fn look_a_head(&mut self, i: i32) -> &Token {
        // ensures the buffer has the element we want to look ahead
        while self.buffer.len() < i as usize {
            self.buffer.push_back(self.tokenizer.next_token());
        }
        assert!(self.buffer.len() >= i as usize);

        let mut cnt = 1;
        for t in &self.buffer {
            if cnt == i {
                return t;
            }
            cnt += 1;
        }

        unreachable!();  // should never reach this
    }

    /*
     * Consumes the next token from the tokeniser or the buffer if not empty.
     */
    fn next_token(&mut self) {
        if !self.buffer.is_empty() {
            self.token = Rc::new(self.buffer.pop_front().unwrap());
        } else {
            self.token = Rc::new(self.tokenizer.next_token());
        }
    }

    /*
     * If the current token is equals to the expected one, then skip it, otherwise report an error.
     * Returns the expected token or null if an error occurred.
     */
    fn expect(&mut self, expected: &[TokenClass]) {
        for e in expected {
            if e.clone() == self.token.token_class {
                self.next_token();
                return;
            }
        }

        self.error(expected);
    }

    /*
     * Returns true if the current token is equals to any of the expected ones.
     */
    fn accept(&mut self, expected: &[TokenClass]) -> bool {
        let mut result = false;
        for e in expected {
            result |= e.clone() == self.token.token_class;
        }
        return result;
    }

    pub fn parse(&mut self) -> Program {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Program {
        self.parse_includes();
        let struct_decls = self.parse_struct_decls();
        let var_decls = self.parse_var_decls();
        let fun_decls = self.parse_fun_decls();
        self.expect(&[TokenClass::EOF]);
        Program::new(struct_decls, var_decls,fun_decls)
    }

    // includes are ignored, so does not need to return an AST node
    fn parse_includes(&mut self) {
        if self.accept(&[TokenClass::INCLUDE]) {
            self.next_token();
            self.expect(&[TokenClass::STRINGLITERAL]);
            self.parse_includes();
        }
    }

    fn parse_struct_decls(&mut self) -> Vec<StructTypeDecl> {
        let mut struct_decls = Vec::new();
        return_if!(!self.accept(&[TokenClass::STRUCT]) || !(self.look_a_head(2).token_class == TokenClass::LBRA); struct_decls);

        self.next_token();
        let mut struct_type_name = String::from("");
        if self.accept(&[TokenClass::IDENTIFIER]) {
            struct_type_name = self.token.data.to_owned();
            self.next_token();
        }
        let struct_type = StructType::new(struct_type_name);

        self.expect(&[TokenClass::LBRA]);

        let var_decls = self.parse_var_decls();

        self.expect(&[TokenClass::RBRA]);
        self.expect(&[TokenClass::SC]);

        struct_decls.push(StructTypeDecl::new(struct_type, var_decls));
        struct_decls.append(&mut self.parse_struct_decls());
        struct_decls
    }

    fn parse_var_decls(&mut self) -> Vec<VarDecl> {
        let mut var_decls = Vec::new();
        return_if!(!self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]); var_decls);

        let tmp;
        if self.accept(&[TokenClass::STRUCT]) {
            tmp = if self.look_a_head(2).token_class == TokenClass::ASTERIX { self.look_a_head(4).token_class.clone() } else { self.look_a_head(3).token_class.clone() };
        } else {
            tmp = if self.look_a_head(1).token_class == TokenClass::ASTERIX { self.look_a_head(3).token_class.clone() } else { self.look_a_head(2).token_class.clone() };
        }
        return_if!(tmp != TokenClass::SC && tmp != TokenClass::LSBR; var_decls);

        let mut a_type = self.parse_type();

        let mut type_name= String::from("");
        if self.accept(&[TokenClass::IDENTIFIER]) {
            type_name = self.token.data.to_owned();
            self.next_token();
        }

        if self.accept(&[TokenClass::SC]) {
            self.next_token();
        } else {
            let mut i = 0;
            self.expect(&[TokenClass::LSBR]);
            if self.accept(&[TokenClass::INTLITERAL]) {
                i = self.token.data.parse::<i32>().unwrap();
                self.next_token();
            }
            self.expect(&[TokenClass::RSBR]);
            self.expect(&[TokenClass::SC]);
            a_type = Box::new(ArrayType::new(a_type, i));
        }

        var_decls.push(VarDecl::new(a_type, type_name));
        var_decls.append(&mut self.parse_var_decls());
        var_decls
    }

    fn parse_fun_decls(&mut self) -> Vec<FunDecl> {
        let mut fun_decls = Vec::new();
        return_if!(!self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]); fun_decls);

        let tmp;
        if self.accept(&[TokenClass::STRUCT]) {
            tmp = if self.look_a_head(2).token_class == TokenClass::ASTERIX { self.look_a_head(4).token_class.clone() } else { self.look_a_head(3).token_class.clone() };
        } else {
            tmp = if self.look_a_head(1).token_class == TokenClass::ASTERIX { self.look_a_head(3).token_class.clone() } else { self.look_a_head(2).token_class.clone() };
        }
        return_if!(tmp != TokenClass::LPAR; fun_decls);

        let a_type = self.parse_type();
        let mut fun_name= String::from("");
        if self.accept(&[TokenClass::IDENTIFIER]){
            fun_name = self.token.data.to_owned();
            self.next_token();
        }

        self.expect(&[TokenClass::LPAR]);
        let params = self.parse_params();

        self.expect(&[TokenClass::RPAR]);
        let block = self.parse_block();

        fun_decls.push(FunDecl::new(a_type, fun_name, params, block));
        fun_decls.append(&mut self.parse_fun_decls());
        fun_decls
    }

    fn parse_params(&mut self) -> Vec<VarDecl> {
        let mut params = Vec::new();
        return_if!(!self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]); params);

        loop {
            if self.accept(&[TokenClass::COMMA]) { self.next_token(); }
            let a_type = self.parse_type();
            let mut var_name = String::from("");
            if self.accept(&[TokenClass::IDENTIFIER]) {
                var_name = self.token.data.to_owned();
                self.next_token();
            }
            params.push(VarDecl::new(a_type, var_name));

            if !self.accept(&[TokenClass::COMMA]) { break; }
        }
        params
    }

    fn parse_block(&mut self) -> Block {

        self.expect(&[TokenClass::LBRA]);

        let var_decls = self.parse_var_decls();
        let mut stmts = Vec::new();

        while !self.accept(&[TokenClass::RBRA, TokenClass::EOF]) { stmts.push(self.parse_stmt()) };
        self.expect(&[TokenClass::RBRA]);
        Block::new(var_decls, stmts)
    }

    fn parse_stmt(&mut self) -> Box<dyn Stmt> {
        return if self.accept(&[TokenClass::LBRA]) {
            Box::new(self.parse_block())
        } else if self.accept(&[TokenClass::WHILE]) {
            self.next_token();
            self.expect(&[TokenClass::LPAR]);
            let expr = self.parse_exp();
            self.expect(&[TokenClass::RPAR]);
            let stmt = self.parse_stmt();
            While::new(expr, stmt)
        } else if self.accept(&[TokenClass::IF]) {
            self.next_token();
            self.expect(&[TokenClass::LPAR]);
            let expr = self.parse_exp();
            self.expect(&[TokenClass::RPAR]);
            let stmt1 = self.parse_stmt();
            let mut stmt2 = None;

            if self.accept(&[TokenClass::ELSE]) {
                self.next_token();
                stmt2 = Some(self.parse_stmt());
            }
            If::new(expr, stmt1, stmt2)
        } else if self.accept(&[TokenClass::RETURN]) {
            let mut expr = None;
            self.next_token();

            if self.accept(&[TokenClass::SC]) {
                self.next_token();
            } else {
                expr = Some(self.parse_exp());
                self.expect(&[TokenClass::SC]);
            }
            Return::new(expr)
        } else {
            let expr1 = self.parse_exp();
            if self.accept(&[TokenClass::ASSIGN]) {
                self.next_token();
                let expr2 = self.parse_exp();
                self.expect(&[TokenClass::SC]);
                Assign::new(expr1, expr2)
            } else {
                self.expect(&[TokenClass::SC]);
                ExprStmt::new(expr1)
            }
        }
    }

    fn parse_exp(&mut self) -> Box<dyn Expr> {
        let mut lhs = self.parse_term().unwrap();
        while self.accept(&[TokenClass::DOT, TokenClass::LSBR, TokenClass::EQ, TokenClass::NE, TokenClass::LT, TokenClass::GT, TokenClass::LE, TokenClass::GE,
            TokenClass::PLUS, TokenClass::MINUS, TokenClass::ASTERIX, TokenClass::DIV, TokenClass::REM, TokenClass::LOGAND, TokenClass::LOGOR]) {

            // let mut pre = 0;
            // let it = (&lhs).as_any();
            // if let Some(lhs) = it.downcast_ref::<BinOp>() {
            //     pre = Parser::parse_pre(lhs.op);
            //     if let (4, Some(expr1)) = (pre, lhs.expr1.as_any().downcast_ref::<IntLiteral>()) {
            //         if expr1.i == 0 { pre = 2; }
            //     }
            // } else if let Some(_) = it.downcast_ref::<TypecastExpr>() { pre = 2; }
            // else if let Some(_) = it.downcast_ref::<ValueAtExpr>() { pre = 2; }
            // else if let Some(_) = it.downcast_ref::<AddressOfExpr>() { pre = 2; }
            //
            // if lhs.get_is_grouped() { pre = 0; }

            if self.accept(&[TokenClass::DOT]) {
                self.next_token();
                let mut name = String::from("");
                if self.accept(&[TokenClass::IDENTIFIER]) {
                    name = self.token.data.to_owned();
                    self.next_token();
                }

                // if pre != 0 {
                //     if let Some(cast_lhs) = it.downcast_ref::<TypecastExpr>() {
                //         lhs = TypecastExpr::new(cast_lhs.typecast_type.into(), FieldAccessExpr::new(cast_lhs.expr.into(), name));
                //     } else if let Some(cast_lhs) = it.downcast_ref::<ValueAtExpr>() {
                //         lhs = ValueAtExpr::new(FieldAccessExpr::new(cast_lhs.expr.into(), name));
                //     } else if let Some(cast_lhs) = it.downcast_ref::<AddressOfExpr>() {
                //         lhs = AddressOfExpr::new(FieldAccessExpr::new(cast_lhs.expr.into(), name));
                //     } else if let Some(cast_lhs) = it.downcast_ref::<BinOp>() {
                //         lhs = BinOp::new(cast_lhs.expr1.into(), cast_lhs.op, FieldAccessExpr::new(cast_lhs.expr2.into(), name));
                //     }
                // } else {
                //     lhs = FieldAccessExpr::new( lhs, name);
                // }
                lhs = FieldAccessExpr::new( lhs, name);
            } else if self.accept(&[TokenClass::LSBR]) {

                self.next_token();
                let rhs = self.parse_exp();
                self.expect(&[TokenClass::RSBR]);

                // if pre != 0 {
                //     if let Some(&cast_lhs) = it.downcast_ref::<TypecastExpr>() {
                //         lhs = TypecastExpr::new(cast_lhs.typecast_type, ArrayAccessExpr::new(cast_lhs.expr, rhs));
                //     } else if let Some(&cast_lhs) = it.downcast_ref::<ValueAtExpr>() {
                //         lhs = ValueAtExpr::new(ArrayAccessExpr::new(cast_lhs.expr, rhs));
                //     } else if let Some(&cast_lhs) = it.downcast_ref::<AddressOfExpr>() {
                //         lhs = AddressOfExpr::new(ArrayAccessExpr::new(cast_lhs.expr, rhs));
                //     } else if let Some(&cast_lhs) = it.downcast_ref::<BinOp>() {
                //         if let Some(&expr2) = (&cast_lhs.expr2).as_any().downcast_ref::<BinOp>() {
                //             let new_expr2 = BinOp::new(expr2.expr1, expr2.op, ArrayAccessExpr::new(expr2.expr2, rhs));
                //             lhs = BinOp::new(cast_lhs.expr1, cast_lhs.op, new_expr2);
                //         } else {
                //             lhs = BinOp::new(cast_lhs.expr1, cast_lhs.op, ArrayAccessExpr::new(cast_lhs.expr2, rhs));
                //         }
                //     }
                // } else {
                //     lhs = ArrayAccessExpr::new(lhs.into(), rhs);
                // }
                lhs = ArrayAccessExpr::new(lhs, rhs);
            } else {
                let op = self.parse_op().unwrap();
                self.next_token();

                let rhs = self.parse_term().unwrap();

                // if let (true, true, Some(&case_lhs)) = (pre != 0, pre > Parser::parse_pre(op), it.downcast_ref::<BinOp>()) {
                //     lhs = BinOp::new(case_lhs.expr1, case_lhs.op, BinOp::new(case_lhs.expr2, op, rhs));
                // } else {
                //     lhs = BinOp::new(lhs.into(), op, rhs);
                // }
                lhs = BinOp::new(lhs, op, rhs);
            }
        }
        lhs
    }

    fn parse_term(&mut self) -> Option<Box<dyn Expr>> {
        if self.accept(&[TokenClass::LPAR]) {
            self.next_token();
            return if self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]) {
                let a_type = self.parse_type();
                self.expect(&[TokenClass::RPAR]);
                let expr = self.parse_term().unwrap();
                Some(TypecastExpr::new(a_type, expr))
            } else {
                let mut expr = self.parse_exp();
                self.expect(&[TokenClass::RPAR]);
                expr.set_is_grouped(true);
                Some(expr)
            }
        } else if self.accept(&[TokenClass::IDENTIFIER]) {
            let name = self.token.data.to_owned();
            self.next_token();

            if self.accept(&[TokenClass::LPAR]) {
                let mut exprs = Vec::new();
                if self.look_a_head(1).token_class != TokenClass::RPAR {
                    loop {
                        self.next_token();
                        exprs.push(self.parse_exp());
                        if !self.accept(&[TokenClass::COMMA]) { break; }
                    }
                } else {
                    self.next_token();
                }
                self.expect(&[TokenClass::RPAR]);
                return Some(FunCallExpr::new(name,exprs));
            }
            return Some(VarExpr::new(name));
        } else if self.accept(&[TokenClass::MINUS, TokenClass::PLUS, TokenClass::ASTERIX, TokenClass::AND]) {
            let t = self.token.token_class.clone();

            self.next_token();
            let expr = self.parse_term().unwrap();

            return if t == TokenClass::MINUS {
                Some(BinOp::new(IntLiteral::new("0".to_owned()), Op::SUB, expr))
            } else if t == TokenClass::PLUS {
                Some(BinOp::new(IntLiteral::new("0".to_owned()), Op::ADD, expr))
            } else if t == TokenClass::ASTERIX {
                Some(ValueAtExpr::new(expr))
            } else {
                Some(AddressOfExpr::new(expr))
            }
        } else if self.accept(&[TokenClass::SIZEOF]) {
            self.next_token();
            self.expect(&[TokenClass::LPAR]);

            let a_type = self.parse_type();

            self.expect(&[TokenClass::RPAR]);

            return Some(SizeOfExpr::new(a_type));
        } else if self.accept(&[TokenClass::INTLITERAL, TokenClass::CHARLITERAL, TokenClass::STRINGLITERAL]) {
            let expr: Box<dyn Expr>;
            if self.token.token_class == TokenClass::INTLITERAL {
                expr = IntLiteral::new(self.token.data.to_owned());
            } else if self.token.token_class == TokenClass::CHARLITERAL {
                expr = ChrLiteral::new(self.token.data.to_owned());
            } else {
                expr = StrLiteral::new(self.token.data.to_owned());
            }

            self.next_token();
            return Some(expr);
        } else {
            self.expect(&[TokenClass::LPAR, TokenClass::IDENTIFIER,
                   TokenClass::MINUS, TokenClass::PLUS, TokenClass::ASTERIX, TokenClass::AND,
                   TokenClass::SIZEOF, TokenClass::INTLITERAL, TokenClass::CHARLITERAL, TokenClass::STRINGLITERAL]);
            self.next_token();
            None
        }
    }

    fn parse_type(&mut self) -> Box<dyn Type> {
        let mut a_type: Option<Box<dyn Type>> = None;
        if self.accept(&[TokenClass::STRUCT]) {
            self.next_token();
            if self.accept(&[TokenClass::IDENTIFIER]) {
                a_type = Some(Box::new(StructType::new(self.token.data.to_owned())));
                self.next_token();
            }
        } else {
            if self.accept(&[TokenClass::INT]) {
                a_type = Some(Box::new(BaseType::INT));
                self.next_token();
            } else if self.accept(&[TokenClass::VOID]) {
                a_type = Some(Box::new(BaseType::VOID));
                self.next_token();
            } else if self.accept(&[TokenClass::CHAR]) {
                a_type = Some(Box::new(BaseType::CHAR));
                self.next_token();
            }
        }

        if self.accept(&[TokenClass::ASTERIX]) {
            self.next_token();
            return PointerType::new(a_type.unwrap());
        }
        a_type.unwrap()
    }

    fn parse_op(&self) -> Option<Op> {
        match self.token.token_class {
            TokenClass::EQ => Some(Op::EQ),
            TokenClass::NE => Some(Op::NE),
            TokenClass::LT => Some(Op::LT),
            TokenClass::GT => Some(Op::GT),
            TokenClass::LE => Some(Op::LE),
            TokenClass::GE => Some(Op::GE),
            TokenClass::PLUS => Some(Op::ADD),
            TokenClass::MINUS => Some(Op::SUB),
            TokenClass::ASTERIX => Some(Op::MUL),
            TokenClass::DIV => Some(Op::DIV),
            TokenClass::REM => Some(Op::MOD),
            TokenClass::LOGAND => Some(Op::AND),
            TokenClass::LOGOR => Some(Op::OR),
            _ => None
        }
    }

    fn parse_pre(op: Op) -> i32 {
        match op {
            Op::MUL | Op::DIV | Op::MOD => 3,
            Op::ADD | Op::SUB => 4,
            Op::LT  | Op::GT | Op::LE | Op::GE => 5,
            Op::EQ  | Op::NE => 6,
            Op::AND => 7,
            _ => 8
        }
    }
}