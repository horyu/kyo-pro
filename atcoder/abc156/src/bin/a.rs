#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        r: usize
    };
    println!("{}", if n >= 10 { r } else { r + 100 * (10 - n) });
}
