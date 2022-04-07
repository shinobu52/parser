use std::io;

use parser::{parser::Ast, error::show_trace};
use parser::interpreter::Interpreter;

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    // インタプリタを用意しておく
    let mut interp = Interpreter::new();

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        // ユーザの入力を取得する
        if let Some(Ok(line)) = lines.next() {
            let ast = match line.parse::<Ast>() {
                Ok(ast) => ast,
                Err(e) => {
                    e.show_diagnostic(&line);
                    show_trace(e);
                    continue
                },
            };
            // インタプリタでevalする
            let n = match interp.eval(&ast) {
                Ok(n) => n,
                Err(e) => {
                    e.show_diagnostic(&line);
                    show_trace(e);
                    continue
                },
            };

            println!("{:?}", n);
        } else {
            break;
        }
    }
}

/// プロンプトを表示してユーザの入力を促す
fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};

    let stdout = stdout();
    let mut stdout = stdout.lock();
    
    stdout.write(s.as_bytes())?;
    stdout.flush()
}
