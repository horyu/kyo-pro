#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

#[allow(unused_must_use)]
fn main() {
    use std::io::{self, Read, Write};
    let mut buf = [b'\n'; 4];
    io::stdin().read(&mut buf[0..3]);
    io::stdout().write(&buf);
}
