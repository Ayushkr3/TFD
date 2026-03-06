use crate::lexer::{Token, TokenType};

// ─────────────────────────────────────────────
//  AST — all types live here, no extra module
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Char,
    Bool,
    String,
}

#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    CharLit(char),
    StrLit(String),
    Ident(String),

    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOpKind,
        operand: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Index {
        name: String,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add, Sub, Mul, Div, Mod,
    Eq, NotEq, Lt, Gt, LtEq, GtEq,
    And, Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOpKind {
    Neg,
    PreInc, PreDec,
    PostInc, PostDec,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Type,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// int x;  or  int x = expr;
    VarDecl {
        ty: Type,
        name: String,
        init: Option<Expr>,
    },
    /// int arr[N];
    ArrayDecl {
        ty: Type,
        name: String,
        size: Expr,
    },
    /// int foo(params) { body }
    FuncDecl {
        ty: Type,
        name: String,
        params: Vec<Param>,
        body: Vec<Stmt>,
    },
    /// x = expr;
    Assign {
        name: String,
        value: Expr,
    },
    /// arr[i] = expr;
    IndexAssign {
        name: String,
        index: Expr,
        value: Expr,
    },
    /// i++;  i--;
    IncDec {
        name: String,
        op: UnaryOpKind,
    },
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Box<Stmt>>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    ExprStmt(Expr),       // standalone function call
    Block(Vec<Stmt>),
}

/// Top-level output — pass this to your next stage
#[derive(Debug)]
pub struct Program {
    pub includes: Vec<String>,            // e.g. ["iostream", "string"]
    pub defines:  Vec<(String, String)>,  // e.g. [("PI", "3.14")]
    pub stmts:    Vec<Stmt>,
}

// ─────────────────────────────────────────────
//  Parser
// ─────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn lookahead(&self) -> &TokenType {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos].token_type
        } else {
            &TokenType::EOF
        }
    }

    fn peek_ahead(&self, offset: usize) -> &TokenType {
        let idx = self.pos + offset;
        if idx < self.tokens.len() { &self.tokens[idx].token_type } else { &TokenType::EOF }
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() { self.pos += 1; }
    }

    fn current_line(&self) -> usize {
        if self.pos < self.tokens.len() { self.tokens[self.pos].line } else { 0 }
    }

    fn current_text(&self) -> String {
        if self.pos < self.tokens.len() { self.tokens[self.pos].lexeme.clone() } else { String::new() }
    }

    fn match_token(&mut self, expected: TokenType) -> Result<(), String> {
        if *self.lookahead() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Syntax error at line {}: expected {:?}, found {:?}",
                self.current_line(), expected, self.lookahead()
            ))
        }
    }

    fn take_identifier(&mut self) -> Result<String, String> {
        if let TokenType::Identifier = self.lookahead() {
            let name = self.current_text();
            self.advance();
            Ok(name)
        } else {
            Err(format!(
                "Syntax error at line {}: expected identifier, found {:?}",
                self.current_line(), self.lookahead()
            ))
        }
    }

    // ── Public entry point ────────────────────

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let (includes, defines) = self.parse_preprocessor_list()?;
        let stmts = self.parse_stmt_list()?;
        self.match_token(TokenType::EOF)?;
        Ok(Program { includes, defines, stmts })
    }

    // ── Preprocessor ─────────────────────────

    fn parse_preprocessor_list(&mut self) -> Result<(Vec<String>, Vec<(String, String)>), String> {
        let mut includes = vec![];
        let mut defines  = vec![];
        loop {
            match self.lookahead() {
                TokenType::Include => {
                    self.advance();
                    match self.lookahead() {
                        TokenType::StringLiteral => {
                            includes.push(self.current_text());
                            self.advance();
                        }
                        TokenType::LessThan => {
                            self.advance();
                            let mut hdr = String::new();
                            while !matches!(self.lookahead(), TokenType::GreaterThan | TokenType::EOF) {
                                hdr.push_str(&self.current_text());
                                self.advance();
                            }
                            self.match_token(TokenType::GreaterThan)?;
                            includes.push(hdr);
                        }
                        _ => {}
                    }
                }
                TokenType::Define => {
                    self.advance();
                    let name = self.take_identifier()?;
                    let val = if !matches!(self.lookahead(),
                        TokenType::EOF | TokenType::Include | TokenType::Define)
                    {
                        let v = self.current_text(); self.advance(); v
                    } else { String::new() };
                    defines.push((name, val));
                }
                _ => break,
            }
        }
        Ok((includes, defines))
    }

    // ── Statement list ────────────────────────

    fn parse_stmt_list(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        loop {
            match self.lookahead() {
                TokenType::Int | TokenType::Float | TokenType::Char
                | TokenType::Bool | TokenType::String
                | TokenType::Identifier | TokenType::If | TokenType::While
                | TokenType::For | TokenType::Return | TokenType::LeftBrace => {
                    stmts.push(self.parse_stmt()?);
                }
                _ => break,
            }
        }
        Ok(stmts)
    }

    // ── Single statement ──────────────────────

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.lookahead() {
            TokenType::Int | TokenType::Float | TokenType::Char
            | TokenType::Bool | TokenType::String => self.parse_decl(),

            TokenType::Identifier => match self.peek_ahead(1) {
                TokenType::LeftParen => {
                    let call = self.parse_func_call_expr()?;
                    self.match_token(TokenType::Semicolon)?;
                    Ok(Stmt::ExprStmt(call))
                }
                TokenType::Assign => {
                    let s = self.parse_assign_stmt()?;
                    self.match_token(TokenType::Semicolon)?;
                    Ok(s)
                }
                TokenType::LeftBracket => {
                    let s = self.parse_index_assign()?;
                    self.match_token(TokenType::Semicolon)?;
                    Ok(s)
                }
                TokenType::Increment => {
                    let name = self.take_identifier()?;
                    self.advance();
                    self.match_token(TokenType::Semicolon)?;
                    Ok(Stmt::IncDec { name, op: UnaryOpKind::PostInc })
                }
                TokenType::Decrement => {
                    let name = self.take_identifier()?;
                    self.advance();
                    self.match_token(TokenType::Semicolon)?;
                    Ok(Stmt::IncDec { name, op: UnaryOpKind::PostDec })
                }
                _ => Err(format!(
                    "Syntax error at line {}: unexpected token after identifier: {:?}",
                    self.current_line(), self.peek_ahead(1)
                )),
            },

            TokenType::If    => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::For   => self.parse_for(),

            TokenType::Return => {
                self.advance();
                if *self.lookahead() == TokenType::Semicolon {
                    self.advance();
                    Ok(Stmt::Return(None))
                } else {
                    let e = self.parse_expr()?;
                    self.match_token(TokenType::Semicolon)?;
                    Ok(Stmt::Return(Some(e)))
                }
            }

            TokenType::LeftBrace => Ok(Stmt::Block(self.parse_block()?)),

            _ => Err(format!(
                "Syntax error at line {}: unexpected token {:?}",
                self.current_line(), self.lookahead()
            )),
        }
    }

    // ── Declarations ──────────────────────────

    fn parse_decl(&mut self) -> Result<Stmt, String> {
        let ty   = self.parse_type()?;
        let name = self.take_identifier()?;
        match self.lookahead() {
            TokenType::Semicolon => { self.advance(); Ok(Stmt::VarDecl { ty, name, init: None }) }
            TokenType::Assign => {
                self.advance();
                let init = self.parse_expr()?;
                self.match_token(TokenType::Semicolon)?;
                Ok(Stmt::VarDecl { ty, name, init: Some(init) })
            }
            TokenType::LeftBracket => {
                self.advance();
                let size = self.parse_expr()?;
                self.match_token(TokenType::RightBracket)?;
                self.match_token(TokenType::Semicolon)?;
                Ok(Stmt::ArrayDecl { ty, name, size })
            }
            TokenType::LeftParen => {
                let (params, body) = self.parse_function()?;
                Ok(Stmt::FuncDecl { ty, name, params, body })
            }
            _ => Err(format!(
                "Syntax error at line {}: expected ';', '=', '[', or '(' after identifier, found {:?}",
                self.current_line(), self.lookahead()
            )),
        }
    }

    fn parse_function(&mut self) -> Result<(Vec<Param>, Vec<Stmt>), String> {
        self.match_token(TokenType::LeftParen)?;
        let params = self.parse_param_list()?;
        self.match_token(TokenType::RightParen)?;
        let body = self.parse_block()?;
        Ok((params, body))
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, String> {
        let mut params = vec![];
        if !matches!(self.lookahead(),
            TokenType::Int | TokenType::Float | TokenType::Char
            | TokenType::Bool | TokenType::String) { return Ok(params); }
        params.push(Param { ty: self.parse_type()?, name: self.take_identifier()? });
        while *self.lookahead() == TokenType::Comma {
            self.advance();
            params.push(Param { ty: self.parse_type()?, name: self.take_identifier()? });
        }
        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let ty = match self.lookahead() {
            TokenType::Int    => Type::Int,
            TokenType::Float  => Type::Float,
            TokenType::Char   => Type::Char,
            TokenType::Bool   => Type::Bool,
            TokenType::String => Type::String,
            _ => return Err(format!(
                "Syntax error at line {}: expected type keyword, found {:?}",
                self.current_line(), self.lookahead()
            )),
        };
        self.advance();
        Ok(ty)
    }

    // ── Assignment helpers ────────────────────

    fn parse_assign_stmt(&mut self) -> Result<Stmt, String> {
        let name = self.take_identifier()?;
        self.match_token(TokenType::Assign)?;
        let value = self.parse_expr()?;
        Ok(Stmt::Assign { name, value })
    }

    fn parse_index_assign(&mut self) -> Result<Stmt, String> {
        let name = self.take_identifier()?;
        self.match_token(TokenType::LeftBracket)?;
        let index = self.parse_expr()?;
        self.match_token(TokenType::RightBracket)?;
        self.match_token(TokenType::Assign)?;
        let value = self.parse_expr()?;
        Ok(Stmt::IndexAssign { name, index, value })
    }

    // ── Control flow ──────────────────────────

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.match_token(TokenType::If)?;
        self.match_token(TokenType::LeftParen)?;
        let condition = self.parse_expr()?;
        self.match_token(TokenType::RightParen)?;
        let then_block = self.parse_block()?;
        let else_block = if *self.lookahead() == TokenType::Else {
            self.advance();
            if *self.lookahead() == TokenType::If {
                Some(vec![self.parse_if()?])   // else-if chain
            } else {
                Some(self.parse_block()?)
            }
        } else { None };
        Ok(Stmt::If { condition, then_block, else_block })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.match_token(TokenType::While)?;
        self.match_token(TokenType::LeftParen)?;
        let condition = self.parse_expr()?;
        self.match_token(TokenType::RightParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.match_token(TokenType::For)?;
        self.match_token(TokenType::LeftParen)?;

        // Init
        let init: Option<Box<Stmt>> = match self.lookahead() {
            TokenType::Int | TokenType::Float | TokenType::Char
            | TokenType::Bool | TokenType::String => {
                let ty   = self.parse_type()?;
                let name = self.take_identifier()?;
                let init_expr = if *self.lookahead() == TokenType::Assign {
                    self.advance(); Some(self.parse_expr()?)
                } else { None };
                Some(Box::new(Stmt::VarDecl { ty, name, init: init_expr }))
            }
            TokenType::Identifier => Some(Box::new(self.parse_assign_stmt()?)),
            _ => None,
        };
        self.match_token(TokenType::Semicolon)?;

        // Condition
        let condition = if *self.lookahead() != TokenType::Semicolon {
            Some(self.parse_expr()?)
        } else { None };
        self.match_token(TokenType::Semicolon)?;

        // Update
        let update: Option<Box<Stmt>> = if *self.lookahead() != TokenType::RightParen {
            match self.peek_ahead(1) {
                TokenType::Increment => {
                    let name = self.take_identifier()?; self.advance();
                    Some(Box::new(Stmt::IncDec { name, op: UnaryOpKind::PostInc }))
                }
                TokenType::Decrement => {
                    let name = self.take_identifier()?; self.advance();
                    Some(Box::new(Stmt::IncDec { name, op: UnaryOpKind::PostDec }))
                }
                _ => Some(Box::new(self.parse_assign_stmt()?)),
            }
        } else { None };

        self.match_token(TokenType::RightParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::For { init, condition, update, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.match_token(TokenType::LeftBrace)?;
        let stmts = self.parse_stmt_list()?;
        self.match_token(TokenType::RightBrace)?;
        Ok(stmts)
    }

    // ── Expressions ───────────────────────────

    fn parse_expr(&mut self) -> Result<Expr, String> { self.parse_logical() }

    fn parse_logical(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while matches!(self.lookahead(), TokenType::LogicalAnd | TokenType::LogicalOr) {
            let op = match self.lookahead() {
                TokenType::LogicalAnd => BinOpKind::And,
                _                     => BinOpKind::Or,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        while matches!(self.lookahead(),
            TokenType::Equal | TokenType::NotEqual | TokenType::LessThan
            | TokenType::GreaterThan | TokenType::LessEqual | TokenType::GreaterEqual)
        {
            let op = match self.lookahead() {
                TokenType::Equal        => BinOpKind::Eq,
                TokenType::NotEqual     => BinOpKind::NotEq,
                TokenType::LessThan     => BinOpKind::Lt,
                TokenType::GreaterThan  => BinOpKind::Gt,
                TokenType::LessEqual    => BinOpKind::LtEq,
                _                       => BinOpKind::GtEq,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while matches!(self.lookahead(), TokenType::Plus | TokenType::Minus) {
            let op = if *self.lookahead() == TokenType::Plus { BinOpKind::Add } else { BinOpKind::Sub };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while matches!(self.lookahead(), TokenType::Multiply | TokenType::Divide | TokenType::Modulo) {
            let op = match self.lookahead() {
                TokenType::Multiply => BinOpKind::Mul,
                TokenType::Divide   => BinOpKind::Div,
                _                   => BinOpKind::Mod,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.lookahead() {
            TokenType::Minus => {
                self.advance();
                Ok(Expr::UnaryOp { op: UnaryOpKind::Neg, operand: Box::new(self.parse_unary()?) })
            }
            TokenType::Increment => {
                self.advance();
                Ok(Expr::UnaryOp { op: UnaryOpKind::PreInc, operand: Box::new(self.parse_unary()?) })
            }
            TokenType::Decrement => {
                self.advance();
                Ok(Expr::UnaryOp { op: UnaryOpKind::PreDec, operand: Box::new(self.parse_unary()?) })
            }
            _ => self.parse_factor(),
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.lookahead() {
            TokenType::Identifier => {
                let name = self.take_identifier()?;
                match self.lookahead() {
                    TokenType::LeftParen => {
                        self.advance();
                        let args = self.parse_arg_list()?;
                        self.match_token(TokenType::RightParen)?;
                        Ok(Expr::Call { name, args })
                    }
                    TokenType::LeftBracket => {
                        self.advance();
                        let index = self.parse_expr()?;
                        self.match_token(TokenType::RightBracket)?;
                        Ok(Expr::Index { name, index: Box::new(index) })
                    }
                    TokenType::Increment => {
                        self.advance();
                        Ok(Expr::UnaryOp { op: UnaryOpKind::PostInc, operand: Box::new(Expr::Ident(name)) })
                    }
                    TokenType::Decrement => {
                        self.advance();
                        Ok(Expr::UnaryOp { op: UnaryOpKind::PostDec, operand: Box::new(Expr::Ident(name)) })
                    }
                    _ => Ok(Expr::Ident(name)),
                }
            }
            TokenType::IntegerLiteral => {
                let v = self.current_text().parse::<i64>().unwrap_or(0);
                self.advance(); Ok(Expr::IntLit(v))
            }
            TokenType::FloatLiteral => {
                let v = self.current_text().parse::<f64>().unwrap_or(0.0);
                self.advance(); Ok(Expr::FloatLit(v))
            }
            TokenType::BoolLiteral => {
                let v = self.current_text() == "true";
                self.advance(); Ok(Expr::BoolLit(v))
            }
            TokenType::CharLiteral => {
                let v = self.current_text().chars().next().unwrap_or('\0');
                self.advance(); Ok(Expr::CharLit(v))
            }
            TokenType::StringLiteral => {
                let v = self.current_text(); self.advance(); Ok(Expr::StrLit(v))
            }
            TokenType::LeftParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.match_token(TokenType::RightParen)?;
                Ok(e)
            }
            _ => Err(format!(
                "Syntax error at line {}: unexpected token in expression: {:?}",
                self.current_line(), self.lookahead()
            )),
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = vec![];
        if *self.lookahead() == TokenType::RightParen { return Ok(args); }
        args.push(self.parse_expr()?);
        while *self.lookahead() == TokenType::Comma {
            self.advance();
            args.push(self.parse_expr()?);
        }
        Ok(args)
    }

    fn parse_func_call_expr(&mut self) -> Result<Expr, String> {
        let name = self.take_identifier()?;
        self.match_token(TokenType::LeftParen)?;
        let args = self.parse_arg_list()?;
        self.match_token(TokenType::RightParen)?;
        Ok(Expr::Call { name, args })
    }
}