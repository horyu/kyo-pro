#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
        lrlr: [(Usize1, Usize1); q]
    };
    let mut ac_counts = vec![0];
    let mut count = 0;
    for i in 0usize..(n - 1) {
        count += (&s[i..=(i + 1)] == "AC") as i32;
        ac_counts.push(count);
    }
    for (l, r) in lrlr {
        println!("{}", ac_counts[r] - ac_counts[l]);
    }
}
