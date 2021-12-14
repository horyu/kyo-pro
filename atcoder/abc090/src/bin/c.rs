#![allow(unused_imports)]
#![allow(clippy::many_single_char_names)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize
    };
    // a <= b
    let (a, b) = [(m, n), (n, m)][(n < m) as usize];
    let rs = match (a, b) {
        (1, 1) => 1,
        (1, d) => d - 2,
        (c, d) => (c - 2) * (d - 2),
    };
    println!("{}", rs);
}
