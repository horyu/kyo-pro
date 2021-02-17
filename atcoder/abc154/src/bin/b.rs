#![allow(unused_must_use)]
use std::io::{self, Read, Write};

fn main() {
    let mut arr = [b'x'; 101];
    let len = io::stdin().bytes().count();
    arr[len - 1] = b'\n';
    io::stdout().write(&arr[..len]);
}
