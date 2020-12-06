#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        k: Usize1,
        mut s: Chars
    };
    s[k] = s[k].to_ascii_lowercase();
    let s: String = s.into_iter().collect();
    println!("{}", s);
}
