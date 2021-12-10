#![allow(unused_imports)]
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut buf = [b'\n'; 11];
    let stdin = stdin();
    let mut handle = stdin.lock();
    handle.read_exact(&mut buf).unwrap();
    buf[0] = b'2';
    buf[1] = b'0';
    buf[2] = b'1';
    buf[3] = b'8';
    stdout().write_all(&buf).unwrap();
}
