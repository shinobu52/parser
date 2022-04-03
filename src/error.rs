use std::{fmt, error::Error as StdError};

use crate::utils::{Annot, Loc};
use crate::lexer::Token;

// 字句解析エラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = Annot<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }

    pub fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LexErrorKind::*;

        let  loc = &self.loc;
        match self.value {
            InvalidChar(c) => write!(f, "{}: invalid char '{}'", loc, c),
            Eof => write!(f, "End of file"),
        }
    }
}

impl StdError for LexError {}

// 構文解析エラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;

        match self {
            UnexpectedToken(tok) => write!(f, "{}: {} is not expected", tok.loc, tok.value),
            NotExpression(tok) => write!(f, "{}: {} is not a start of expression", tok.loc, tok.value),
            NotOperator(tok) => write!(f, "{}: '{}' is not an operator", tok.loc, tok.value),
            UnclosedOpenParen(tok) => write!(f, "{}: '{}' is not closed", tok.loc, tok.value),
            RedundantExpression(tok) => write!(f, "{}: expression after '{}' is redundant", tok.loc, tok.value),
            Eof => write!(f, "End of file"),
        }
    }
}

impl StdError for ParseError {}

/// 字句解析エラーと構文解析エラーを統合するエラー型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Lexer(LexError),
    Parser(ParseError)
}

impl Error {
    /// 診断メッセージを表示する
    fn show_diagnostic(&self, input: &str) {
        use self::Error::*;
        use self::ParseError as P;

        // エラー情報とその位置情報を取り出す。エラーの種類によって位置情報を調整する
        let (e, loc) = match self {
            Lexer(e) => (e, e.loc.close()),
            Parser(e) => {
                let loc = match e {
                    P::UnexpectedToken(Token {loc, ..})
                    | P::NotExpression(Token {loc, ..})
                    | P::NotOperator(Token {loc, ..})
                    | P::UnclosedOpenParen(Token {loc, ..}) => loc.close(),
                    // redundant expressionはトークン以降行末までが余りなのでlocの終了位置を調整する
                    P::RedundantExpression(Token {loc, ..}) => Loc(loc.0, input.len()),
                    // EoFはloc情報を持っていないのでその場で作る
                    P::Eof => Loc(input.len(), input.len() + 1),
                };
                (e, loc)
            },
        };
        // エラー情報を簡単に表示し
        eprintln!("{}", e);
        // エラー位置を指示する
        print_annot(input, loc);
    }
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parser error")
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;

        match self {
            Lexer(lex) => Some(lex),
            Parser(parse) => Some(parse),
        }
    }
}

/// inputに対してlocの位置を強調表示する
fn print_annot(input: &str, loc: Loc) {
    // 入力に対して
    eprintln!("{}", input);
    // 位置情報を分かりやすく示す
    eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0));
}

fn show_trace<E: StdError>(e: E) {
    // エラーがあった場合そのエラーとsourceを全部出力する
    eprintln!("{}", e);
    let mut source = e.source();
    // sourceをすべてたどって表示する
    while let Some(e) = source {
        eprintln!("caused by {}", e);
        source = e.source()
    }
    // エラー表示のあとは次の入力を受け付ける
}
