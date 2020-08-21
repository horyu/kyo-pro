use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut buf = [b'A', b'B', b'C', b'\0', b'\0', b'\0', b'\n'];
    let mut stdin = stdin();
    stdin.lock().read(&mut buf[3..6]);
    stdout().write(&buf);
}
