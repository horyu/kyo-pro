// cargo atcoder submit -f b
// 実際の標準入力の最後にあるLFがtestだとない
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut buf = [0; 32];
    let mut stdin = stdin();
    stdin.lock().read(&mut buf[1..32]);
    buf.reverse();
    buf[31] = b'\n';
    stdout().write(&buf);
}
