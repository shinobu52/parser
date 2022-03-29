use std::io;

use parser::lexer::lex;

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        // ユーザの入力を取得する
        if let Some(Ok(line)) = lines.next() {
            // 字句解析を行う
            let token = lex(&line);
            println!("{:?}", token);
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
