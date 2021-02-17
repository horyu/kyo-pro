#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String,
        _t: String,
        mut a: usize,
        mut b: usize,
        u: String
    };
    if u == s {
        a -= 1;
    } else {
        b -= 1;
    }
    println!("{} {}", a, b);
}
