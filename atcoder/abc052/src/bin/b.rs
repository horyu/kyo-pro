#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let mut max = 0;
    let mut x = 0;
    for c in s {
        x += match c {
            'I' => 1,
            'D' => -1,
            _ => unreachable!(),
        };
        max = std::cmp::max(max, x);
    }
    println!("{}", max);
}
