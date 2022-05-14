use std::collections::VecDeque;
use std::rc::Rc;
use crate::lexer::token::Token;
use crate::{TokenClass, Tokenizer};
use crate::ast::decl::{Program, StructTypeDecl, VarDecl};
use crate::ast::types::StructType;


macro_rules! return_if {
    ( $e:expr ) => {
        if $e { return; }
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

    pub fn parse(&mut self) {
        self.parse_program();
    }

    fn parse_program(&mut self) -> Program {
        self.parse_includes();
        let mut struct_decls = self.parse_struct_decls();
        println!("{:?}",struct_decls.len());
        println!("{:?}",struct_decls.pop().unwrap().struct_type.name);
        self.parse_var_decls();
        self.parse_fun_decls();
        self.expect(&[TokenClass::EOF]);
        Program::new(struct_decls, Vec::new(),Vec::new())
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
        // return_if!(!self.accept(&[TokenClass::STRUCT]) || !(self.look_a_head(2).token_class == TokenClass::LBRA));
        if !self.accept(&[TokenClass::STRUCT]) || !(self.look_a_head(2).token_class == TokenClass::LBRA) {
            return struct_decls;
        }

        self.next_token();
        let mut struct_type_name = String::from("");
        if self.accept(&[TokenClass::IDENTIFIER]) {
            struct_type_name = self.token.data.to_owned();
            self.next_token();
        }
        let structType = StructType::new(struct_type_name);

        self.expect(&[TokenClass::LBRA]);

        self.parse_var_decls();

        self.expect(&[TokenClass::RBRA]);
        self.expect(&[TokenClass::SC]);

        struct_decls.push(StructTypeDecl::new(structType, Vec::new()));
        let mut return_struct_decls = self.parse_struct_decls();
        struct_decls.append(&mut return_struct_decls);
        struct_decls
    }

    fn parse_var_decls(&mut self) {
        return_if!(!self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]));

        let tmp;
        if self.accept(&[TokenClass::STRUCT]) {
            tmp = if self.look_a_head(2).token_class == TokenClass::ASTERIX { self.look_a_head(4).token_class.clone() } else { self.look_a_head(3).token_class.clone() };
        } else {
            tmp = if self.look_a_head(1).token_class == TokenClass::ASTERIX { self.look_a_head(3).token_class.clone() } else { self.look_a_head(2).token_class.clone() };
        }
        return_if!(tmp != TokenClass::SC && tmp != TokenClass::LSBR);

        // List<VarDecl> returnVarDecl;
        self.parse_type();

        let type_name;
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
            // type = new ArrayType(type, i);
        }

        // varDecls.add(new VarDecl(type, type_name));

        self.parse_var_decls();
        // varDecls.addAll(returnVarDecl);
    }

    fn parse_fun_decls(&mut self) {
        return_if!(!self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]));

        let tmp;
        if self.accept(&[TokenClass::STRUCT]) {
            tmp = if self.look_a_head(2).token_class == TokenClass::ASTERIX { self.look_a_head(4).token_class.clone() } else { self.look_a_head(3).token_class.clone() };
        } else {
            tmp = if self.look_a_head(1).token_class == TokenClass::ASTERIX { self.look_a_head(3).token_class.clone() } else { self.look_a_head(2).token_class.clone() };
        }
        return_if!(tmp != TokenClass::LPAR);
        // List<FunDecl> returnFunDecls;

        self.parse_type();
        let fun_name;
        if self.accept(&[TokenClass::IDENTIFIER]){
            fun_name = self.token.data.to_owned();
            self.next_token();
        }

        self.expect(&[TokenClass::LPAR]);
        self.parse_params();

        self.expect(&[TokenClass::RPAR]);
        self.parse_block();

        // funDecls.add(new FunDecl(type,fun_name,varDecls, block));
        self.parse_fun_decls();

        // funDecls.addAll(returnFunDecls);
    }

    fn parse_params(&mut self) {
        return_if!(!self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]));

        loop {
            if self.accept(&[TokenClass::COMMA]) { self.next_token(); }
            self.parse_type();
            let var_name;
            if self.accept(&[TokenClass::IDENTIFIER]) {
                var_name = self.token.data.to_owned();
                self.next_token();
            }
            // varDecls.add(new VarDecl(type, var_name));

            if !self.accept(&[TokenClass::COMMA]) { break; }
        }
    }

    fn parse_block(&mut self) {

        self.expect(&[TokenClass::LBRA]);

        self.parse_var_decls();
        // List<Stmt> stmts = new ArrayList<>();

        while !self.accept(&[TokenClass::RBRA, TokenClass::EOF]) { self.parse_stmt() };
        self.expect(&[TokenClass::RBRA]);
        // return new Block(varDecls, stmts);
    }

    fn parse_stmt(&mut self) {
        if self.accept(&[TokenClass::LBRA]) {
            return self.parse_block();
        } else if self.accept(&[TokenClass::WHILE]) {
            self.next_token();
            self.expect(&[TokenClass::LPAR]);
            self.parse_exp();
            self.expect(&[TokenClass::RPAR]);
            self.parse_stmt();
            // return new While(expr, stmt);
        } else if self.accept(&[TokenClass::IF]) {
            self.next_token();
            self.expect(&[TokenClass::LPAR]);
            self.parse_exp();
            self.expect(&[TokenClass::RPAR]);
            self.parse_stmt();
            // Stmt stmt2 = null;

            if self.accept(&[TokenClass::ELSE]) {
                self.next_token();
                self.parse_stmt();
            }
            // return new If(expr, stmt1, stmt2);
        } else if self.accept(&[TokenClass::RETURN]) {
            // Expr expr = null;
            self.next_token();

            if self.accept(&[TokenClass::SC])  {
                self.next_token();
            } else {
                self.parse_exp();
                self.expect(&[TokenClass::SC]);
            }
            // return new Return(expr);
        } else {
            self.parse_exp();
            if self.accept(&[TokenClass::ASSIGN]) {
                self.next_token();
                self.parse_exp();
                self.expect(&[TokenClass::SC]);
                // return new Assign(expr1, expr2);
            } else {
                self.expect(&[TokenClass::SC]);
                // return new ExprStmt(expr1);
            }
        }
    }

    fn parse_exp(&mut self) {
        self.parse_term();
        while self.accept(&[TokenClass::DOT, TokenClass::LSBR, TokenClass::EQ, TokenClass::NE, TokenClass::LT, TokenClass::GT, TokenClass::LE, TokenClass::GE,
            TokenClass::PLUS, TokenClass::MINUS, TokenClass::ASTERIX, TokenClass::DIV, TokenClass::REM, TokenClass::LOGAND, TokenClass::LOGOR]) {

            // int pre = 0;
            // if(lhs instanceof BinOp)  {
            //     pre = parsePre(((BinOp) lhs).op);
            //     if(pre == 4 && ((BinOp) lhs).expr1 instanceof IntLiteral) {
            //         if(((IntLiteral) ((BinOp) lhs).expr1).i == 0) pre = 2;
            //     }
            // } else if (lhs instanceof TypecastExpr ||
            //     lhs instanceof ValueAtExpr || lhs instanceof AddressOfExpr) pre = 2;
            //
            // if(lhs != null && lhs.isGroup) pre = 0;

            if self.accept(&[TokenClass::DOT]) {
                self.next_token();
                let name;
                if self.accept(&[TokenClass::IDENTIFIER]) {
                    name = self.token.data.to_owned();
                    self.next_token();
                }

                // if(pre != 0)  {
                //     if(lhs instanceof TypecastExpr) {
                //         lhs = new TypecastExpr(((TypecastExpr) lhs).type, new FieldAccessExpr(((TypecastExpr) lhs).expr, name));
                //     } else if(lhs instanceof ValueAtExpr) {
                //         lhs = new ValueAtExpr(new FieldAccessExpr(((ValueAtExpr) lhs).expr, name));
                //     } else if(lhs instanceof AddressOfExpr) {
                //         lhs = new AddressOfExpr(new FieldAccessExpr(((AddressOfExpr) lhs).expr, name));
                //     } else {
                //         lhs = new BinOp(((BinOp) lhs).expr1, ((BinOp) lhs).op, new FieldAccessExpr(((BinOp) lhs).expr2, name));
                //     }
                // }
                // else lhs = new FieldAccessExpr(lhs, name);
            } else if self.accept(&[TokenClass::LSBR]) {

                self.next_token();
                self.parse_exp();
                self.expect(&[TokenClass::RSBR]);

                // if(pre != 0) {
                //     if(lhs instanceof TypecastExpr) {
                //         lhs = new TypecastExpr(((TypecastExpr) lhs).type, new ArrayAccessExpr(((TypecastExpr) lhs).expr, rhs));
                //     } else if(lhs instanceof ValueAtExpr) {
                //         lhs = new ValueAtExpr(new ArrayAccessExpr(((ValueAtExpr) lhs).expr, rhs));
                //     } else if(lhs instanceof AddressOfExpr) {
                //         lhs = new AddressOfExpr(new ArrayAccessExpr(((AddressOfExpr) lhs).expr, rhs));
                //     } else {
                //         if(((BinOp) lhs).expr2 instanceof BinOp) {
                //             BinOp binOp = new BinOp(((BinOp) ((BinOp) lhs).expr2).expr1, ((BinOp) ((BinOp) lhs).expr2).op, new ArrayAccessExpr(((BinOp) ((BinOp) lhs).expr2).expr2, rhs));
                //             lhs = new BinOp(((BinOp) lhs).expr1, ((BinOp) lhs).op, binOp);
                //         } else {
                //             lhs = new BinOp(((BinOp) lhs).expr1, ((BinOp) lhs).op, new ArrayAccessExpr(((BinOp) lhs).expr2, rhs));
                //         }
                //     }
                // }
                // else lhs = new ArrayAccessExpr(lhs, rhs);

            } else {
                // self.parse_op();
                self.next_token();

                self.parse_term();

                // if(pre != 0 && pre > parsePre(op) && lhs instanceof BinOp) lhs = new BinOp(((BinOp) lhs).expr1, ((BinOp) lhs).op, new BinOp(((BinOp) lhs).expr2, op, rhs));
                // else lhs = new BinOp(lhs, op, rhs);
            }
        }
    }

    fn parse_term(&mut self) {
        if self.accept(&[TokenClass::LPAR]) {
            self.next_token();
            if self.accept(&[TokenClass::INT, TokenClass::CHAR, TokenClass::VOID, TokenClass::STRUCT]) {
                self.parse_type();
                self.expect(&[TokenClass::RPAR]);
                self.parse_term();
                // return new TypecastExpr(type, expr);
            } else {
                self.parse_exp();
                self.expect(&[TokenClass::RPAR]);
                // expr.isGroup = true;
                // return expr;
            }
        } else if self.accept(&[TokenClass::IDENTIFIER]) {
            let name = self.token.data.to_owned();
            self.next_token();

            if self.accept(&[TokenClass::LPAR]) {
                // List<Expr> exprs = new ArrayList<>();
                if self.look_a_head(1).token_class != TokenClass::RPAR {
                    loop {
                        self.next_token();
                        self.parse_exp();
                        if !self.accept(&[TokenClass::COMMA]) { break; }
                    }
                } else {
                    self.next_token();
                }
                self.expect(&[TokenClass::RPAR]);
                // return new FunCallExpr(name,exprs);
            }
            // return new VarExpr(name);
        } else if self.accept(&[TokenClass::MINUS, TokenClass::PLUS, TokenClass::ASTERIX, TokenClass::AND]) {
            let t = self.token.token_class.clone();

            self.next_token();
            self.parse_term();

            // if(t == TokenClass.MINUS) {
            //     return new BinOp(new IntLiteral ("0"), Op.SUB, expr);
            // } else if(t == TokenClass.PLUS) {
            //     return new BinOp(new IntLiteral("0"), Op.ADD, expr);
            // } else if(t == TokenClass.ASTERIX) {
            //     return new ValueAtExpr(expr);
            // } else {
            //     return new AddressOfExpr(expr);
            // }
        } else if self.accept(&[TokenClass::SIZEOF]) {
            self.next_token();
            self.expect(&[TokenClass::LPAR]);

            // Type type = parseType();

            self.expect(&[TokenClass::RPAR]);

            // return new SizeOfExpr(type);
        } else if self.accept(&[TokenClass::INTLITERAL, TokenClass::CHARLITERAL, TokenClass::STRINGLITERAL]) {
            // Expr expr;
            // if(token.tokenClass == TokenClass.INT_LITERAL) {
            //     expr = new IntLiteral(token.data);
            // } else if(token.tokenClass == TokenClass.CHAR_LITERAL) {
            //     expr = new ChrLiteral(token.data);
            // } else {
            //     expr = new StrLiteral(token.data);
            // }

            self.next_token();
            // return expr;
        } else {
            self.expect(&[TokenClass::LPAR, TokenClass::IDENTIFIER,
                   TokenClass::MINUS, TokenClass::PLUS, TokenClass::ASTERIX, TokenClass::AND,
                   TokenClass::SIZEOF, TokenClass::INTLITERAL, TokenClass::CHARLITERAL, TokenClass::STRINGLITERAL]);
            self.next_token();
        }
    }

    fn parse_type(&mut self) {
        // Type type = null;
        if self.accept(&[TokenClass::STRUCT]) {
            self.next_token();
            if self.accept(&[TokenClass::IDENTIFIER]) {
                // type = new StructType(token.data);
                self.next_token();
            }
        } else {
            if self.accept(&[TokenClass::INT]) {
                // type = BaseType.INT;
                self.next_token();
            } else if self.accept(&[TokenClass::VOID]) {
                // type = BaseType.VOID;
                self.next_token();
            } else if self.accept(&[TokenClass::CHAR]) {
                // type = BaseType.CHAR;
                self.next_token();
            }
        }

        if self.accept(&[TokenClass::ASTERIX]) {
            self.next_token();
            // return new PointerType(type);
        }
    }
}