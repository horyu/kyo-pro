#![allow(unused_imports)]
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut buf = [b'\n'; 8];
    let stdin = stdin();
    let mut handle = stdin.lock();
    handle.read(&mut buf).unwrap();
    if (buf[0] == buf[6]) && (buf[1] == buf[5]) && (buf[2] == buf[4]) {
        stdout().write_all(b"YES").unwrap();
    } else {
        stdout().write_all(b"NO").unwrap();
    };
}
