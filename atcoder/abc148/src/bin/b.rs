#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };
    let mut rs = String::with_capacity(n * 2);
    for i in 0..n {
        rs.push(s[i]);
        rs.push(t[i]);
    }
    println!("{}", rs);
}
