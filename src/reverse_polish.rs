use crate::parser::{Ast, AstKind, UniOp, UniOpKind, BinOp, BinOpKind};

/// 逆ポーランド記法へのコンパイラを表すデータ型
struct RpnCompiler;

impl RpnCompiler {
    pub fn new() -> Self {
        RpnCompiler
    }

    pub fn compile(&mut self, expr: &Ast) -> String {
        let mut buf = String::new();
        self.compile_inner(expr, &mut buf);
        buf
    }

    fn compile_inner(&mut self, expr: &Ast, buf: &mut String) {
        use self::AstKind::*;

        match expr.value {
            Num(n) => buf.push_str(&n.to_string()),
            UniOp { ref op, ref e } => {
                self.compile_uniop(op, buf);
                self.compile_inner(e, buf)
            },
            BinOp { ref op, ref l, ref r } => {
                self.compile_inner(l, buf);
                buf.push_str(" ");
                self.compile_inner(r, buf);
                buf.push_str(" ");
                self.compile_binop(op, buf)
            }
        }
    }

    fn compile_uniop(&mut self, op: &UniOp, buf: &mut String) {
        use self::UniOpKind::*;

        match op.value {
            Plus => buf.push_str("+"),
            Minus => buf.push_str("-"),
        }
    }

    fn compile_binop(&mut self, op: &BinOp, buf: &mut String) {
        use self::BinOpKind::*;

        match op.value {
            Add => buf.push_str("+"),
            Sub => buf.push_str("-"),
            Mult => buf.push_str("*"),
            Div => buf.push_str("/"),
        }
    }
}

#[test]
fn test_reverse_polish() {
    use crate::utils::Loc;
    use self::RpnCompiler;

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
    let mut compiler = RpnCompiler::new();

    let reverse_polish = compiler.compile(&ast);

    assert_eq!(reverse_polish, "1 2 3 * + -10 -")
}
