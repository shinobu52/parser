use crate::utils::{Annot, Loc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // 数字
    Number(u64),
    // +
    Plus,
    // -
    Minus,
    // *
    Asterisk,
    // /
    Slash,
    // (
    LParen,
    // )
    RParen,
}

// TokenKindにアノテーションを付与したものをTokenとする
pub type Token = Annot<TokenKind>;

// ヘルパーメソッドを定義
impl Token {
    pub fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    pub fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    pub fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    pub fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }

    pub fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    pub fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

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

/// 字句解析器
pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    // 解析結果を保存するベクタ
    let mut tokens = Vec::new();
    // 入力
    let input = input.as_bytes();
    // 位置情報
    let mut pos = 0;

    // サブレキサを読んだ後posを更新するマクロ
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }}
    }

    while pos < input.len() {
        match input[pos] {
            b'0'..=b'9' => lex_a_token!(lex_number(input, pos)),
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, pos)?;
                pos = p;
            }
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

/// posの位置のバイトが期待したものなら、1バイト消費してposを1つ進める
fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    if input.len() <= pos {
        return Err(LexError::eof(Loc(pos, pos)));
    }

    if input[pos] != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos + 1)
        ));
    }
    Ok((b, pos + 1))
}

fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // 以下のようにResult::mapで簡潔に書ける
    consume_byte(input, start, b'+').map(|(_, end)|
        (Token::plus(Loc(start, end)), end)
    )
}

fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)|
        (Token::minus(Loc(start, end)), end)
    )
}

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'*').map(|(_, end)|
        (Token::asterisk(Loc(start, end)), end)
    )
}

fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/').map(|(_, end)|
        (Token::slash(Loc(start, end)), end)
    )
}

fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(').map(|(_, end)|
        (Token::lparen(Loc(start, end)), end)
    )
}

fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')').map(|(_, end)|
        (Token::rparen(Loc(start, end)), end)
    )
}

fn lex_number(input: &[u8], pos: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;

    // 入力に数字が続く限り位置を進める
    let start = pos;
    let pos = recognize_many(input, pos, |b| b"1234567890".contains(&b));

    // 数字の列を数値に変換
    let n = from_utf8(&input[start..pos])
        .unwrap() // start..posの範囲でfrom_utf8は常に成功するためunwrap
        .parse()
        .unwrap(); // 同じ理由
    Ok((Token::number(n, Loc(start, pos)), pos))
}

fn skip_spaces(input: &[u8], pos: usize) -> Result<((), usize), LexError> {
    // 空白が含まれているか判定するのでbyte文字列には' '（スペース）を含める
    let pos = recognize_many(input, pos, |b| b" \n\t".contains(&b));
    Ok(((), pos))
}

fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

#[test]
fn test_lexer() {
    assert_eq!(
        lex("1 + 2 * 3 - -10"),
        Ok(vec![
            Token::number(1, Loc(0, 1)),
            Token::plus(Loc(2, 3)),
            Token::number(2, Loc(4, 5)),
            Token::asterisk(Loc(6, 7)),
            Token::number(3, Loc(8, 9)),
            Token::minus(Loc(10, 11)),
            Token::minus(Loc(12, 13)),
            Token::number(10, Loc(13, 15)),
        ])
    )
}
