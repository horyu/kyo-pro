#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        t: usize,
        ctct: [(usize, usize); n]
    };
    let ct = ctct
        .iter()
        .min_by_key(|(ci, ti)| if *ti <= t { *ci } else { std::usize::MAX })
        .unwrap_or(&(0, std::usize::MAX));
    if ct.1 <= t {
        println!("{}", ct.0);
    } else {
        println!("TLE");
    }
}
