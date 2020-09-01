use std::io::{stdin, stdout, Read, Write};
fn main() {
    let mut buffer = Vec::new();
    let stdin = stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut buffer).unwrap();
    let mut outbuf = Vec::new();
    for &c in buffer.iter().step_by(2) {
        outbuf.push(c);
    }
    outbuf.push(b'\n');
    stdout().write(&outbuf).unwrap();
}
