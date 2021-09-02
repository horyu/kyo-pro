#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use std::{
    collections::{HashMap, HashSet},
    io::stdout,
};

#[allow(unused_must_use)]
fn main() {
    use std::io::{self, Read, Write};
    let mut buf = [0; 6];
    let mut stdin = io::stdin();
    stdin.read(&mut buf);
    let n = buf.iter().filter(|c| **c == b'1').count() as u8;
    let mut write_buf = [b'\n'; 2];
    write_buf[0] = b'0' + n;
    stdout().write_all(&write_buf);
}
