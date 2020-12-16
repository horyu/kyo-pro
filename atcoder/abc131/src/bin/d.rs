#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut abab: [(usize, usize); n]
    };
    abab.sort_by_key(|ab| ab.1);
    abab.push((0, abab[n - 1].1 + 1));
    let mut sum = 0;
    let mut pre_t = 0;
    for (a, b) in abab {
        if (pre_t != b) && (sum > pre_t) {
            println!("No");
            return;
        }
        sum += a;
        pre_t = b;
    }
    println!("Yes");
}
