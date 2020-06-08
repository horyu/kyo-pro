#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: Chars,
        n: usize,
        llrr: [(Usize1, Usize1); n]
    };
    for (l, r) in llrr {
        for i in 0..=((r - l) / 2) {
            s.swap(l + i, r - i);
        }
    }
    let s: String = s.iter().collect();
    println!("{}", s);
}
