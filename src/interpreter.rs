use crate::parser::{Ast, AstKind, UniOp, UniOpKind, BinOp, BinOpKind};
use crate::error::{InterpreterError, InterpreterErrorKind};

/// 評価器を表すデータ型
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: &Ast) -> Result<i64, InterpreterError> {
        use self::AstKind::*;

        match expr.value {
            Num(n) => Ok(n as i64),
            UniOp { ref op, ref e } => {
                let e = self.eval(e)?;
                Ok(self.eval_uniop(op, e))
            },
            BinOp { ref op, ref l, ref r } => {
                let l = self.eval(l)?;
                let r = self.eval(r)?;
                self.eval_binop(op, l, r)
                    .map_err(|e| InterpreterError::new(e, expr.loc.clone()))
            },
        }
    }

    fn eval_uniop(&mut self, op: &UniOp, n: i64) -> i64 {
        use self::UniOpKind::*;

        match op.value {
            Plus => n,
            Minus => -n,
        }
    }

    fn eval_binop(&mut self, op: &BinOp, l: i64, r: i64) -> Result<i64, InterpreterErrorKind> {
        use self::BinOpKind::*;

        match op.value {
            Add => Ok(l + r),
            Sub => Ok(l - r),
            Mult => Ok(l * r),
            Div => {
                if r == 0 {
                    Err(InterpreterErrorKind::DivisionByZero)
                } else {
                    Ok(l / r)
                }
            },
        }
    }
}

#[test]
fn test_interpreter() {
    use crate::utils::Loc;
    
    // 1 + 2 * 3 - -10
    let ast = Ast::binop(
        BinOp::sub(Loc(10, 11)),
        Ast::binop(
            BinOp::add(Loc(2, 3)),
            Ast::num(1, Loc(0, 1)),
            Ast::binop(
                BinOp::new(BinOpKind::Mult, Loc(6, 7)),
                Ast::num(2, Loc(4, 5)),
                Ast::num(3, Loc(8, 9)),
                Loc(4, 9)
            ),
            Loc(0, 9),
        ),
        Ast::uniop(
            UniOp::minus(Loc(12, 13)),
            Ast::num(10, Loc(13, 15)),
            Loc(12, 15)
        ),
        Loc(0, 15)
    );
    let mut interp = Interpreter::new();

    let ans = interp.eval(&ast).unwrap();

    assert_eq!(ans, 17)
}
