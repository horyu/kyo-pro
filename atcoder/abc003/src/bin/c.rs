#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut rr: [usize; n]
    };
    rr.sort();
    let mut c = 0f64;
    let len = rr.len();
    for &r in &rr[(len - k)..len] {
        c = (c + r as f64) / 2.0;
    }
    println!("{}", c);
}
