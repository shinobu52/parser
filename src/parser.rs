use std::iter::Peekable;

use crate::utils::{Annot, Loc};
use crate::lexer::{Token, TokenKind};

/// ASTを表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AstKind {
    /// 数値
    Num(u64),
    /// 単項演算
    UniOp {op: UniOp, e: Box<Ast>},
    /// 二項演算
    BinOp {op: BinOp, l: Box<Ast>, r: Box<Ast>},
}

type Ast = Annot<AstKind>;

impl Ast {
    fn num(n: u64, loc: Loc) -> Self {
        Self::new(AstKind::Num(n), loc)
    }

    fn uniop(op: UniOp, e: Ast, loc: Loc) -> Self {
        Self::new(AstKind::UniOp {op, e: Box::new(e)}, loc)
    }

    fn binop(op: BinOp, l: Ast, r: Ast, loc: Loc) -> Self {
        Self::new(
            AstKind::BinOp {
                op,
                l: Box::new(l),
                r: Box::new(r),
            },
            loc,
        )
    }
}

/// 単項演算子を表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum UniOpKind {
    /// 正号
    Plus,
    /// 負号
    Minus,
}

type UniOp = Annot<UniOpKind>;

impl UniOp {
    fn plus(loc: Loc) -> Self {
        Self::new(UniOpKind::Plus, loc)
    }

    fn minus(loc: Loc) -> Self {
        Self::new(UniOpKind::Minus, loc)
    }
}

/// 二項演算子を表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BinOpKind {
    /// 加算 
    Add,
    /// 減算
    Sub,
    /// 乗算
    Mult,
    /// 除算
    Div,
}

type BinOp = Annot<BinOpKind>;

impl BinOp {
    fn add(loc: Loc) -> Self {
        Self::new(BinOpKind::Add, loc)
    }

    fn sub(loc: Loc) -> Self {
        Self::new(BinOpKind::Sub, loc)
    }

    fn mult(loc: Loc) -> Self {
        Self::new(BinOpKind::Mult, loc)
    }

    fn div(loc: Loc) -> Self {
        Self::new(BinOpKind::Div, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParseError {
    /// 予期しないトークン
    UnexpectedToken(Token),
    /// 式を期待していたのに式でないものがきた
    NotExpression(Token),
    /// 演算子を期待していたのに演算子でないものがきた
    NotOperator(Token),
    /// 括弧が閉じられていない
    UnclosedOpenParen(Token),
    /// 式の解析が終わったのにまだトークンが残っている
    RedundantExpression(Token),
    /// パース途中で入力が終わった
    Eof,
}

fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    // 入力をイテレータにし、Peekableにする
    let mut tokens = tokens.into_iter().peekable();

    // その後、parse_exprを呼んでエラー処理をする
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ret),
    }
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError> 
where
    Tokens: Iterator<Item = Token>,
{
    // parse_exprはparse_expr3を呼ぶ
    parse_expr3(tokens)
}

fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    fn parse_expr3_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens.peek()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Plus => Ok(BinOp::add(tok.loc.clone())),
                TokenKind::Minus => Ok(BinOp::sub(tok.loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }

    parse_left_binop(tokens, parse_expr2, parse_expr3_op)
}

fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    fn parse_expr2_op<Tokens>(tokens: &mut Peekable<Tokens>)
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens.peek()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Asterisk => Ok(BinOp::mult(tok.loc.clone())),
                TokenKind::Minus => Ok(BinOp::div(tok.loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }

    parse_left_binop(tokens, parse_expr1, parse_expr2_op)
}

fn parse_left_binop<Tokens>(
    tokens: &mut Peekable<Tokens>,
    subexpr_parser: fn(&mut Peekable<Tokens>) -> Result<Ast, ParseError>,
    op_parser: fn(&mut Peekable<Tokens>) -> Result<BinOp, ParseError>,
) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut e = subexpr_parser(tokens)?;
    loop {
        match tokens.peek() {
            Some(_) => {
                let op = match op_parser(tokens) {
                    Ok(op) => op,
                    Err(_) => break,
                };
                let r = subexpr_parser(tokens)?;
                let loc = e.loc.merge(&r.loc);
                e = Ast::binop(op, e, r, loc)
            },
            _ => break,
        }
    }
    Ok(e)
}
