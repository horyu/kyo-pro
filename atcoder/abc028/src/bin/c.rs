#![allow(unused_imports)]
#![allow(clippy::many_single_char_names)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize
    };
    println!("{}", std::cmp::max(e + d + a, e + c + b));
}
