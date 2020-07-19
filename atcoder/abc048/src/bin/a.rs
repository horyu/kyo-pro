#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _s0: Chars,
        s1: Chars,
        _s2: Chars
    };
    println!("A{}C", s1[0]);
}
