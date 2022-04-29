#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut rs = 0;
    for (_k, g) in s.into_iter().group_by(|&k| k).into_iter() {
        let size = g.into_iter().count();
        rs += size * (size - 1) / 2;
    }
    println!("{rs}");
}
