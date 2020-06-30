#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        sspp: [(String, usize); n]
    };
    let mut sum = 0;
    let mut max = (String::new(), 0);
    for sp in sspp {
        sum += sp.1;
        if sp.1 > max.1 {
            max = sp;
        }
    }
    println!("{}", if max.1 * 2 > sum { &max.0 } else { "atcoder" });
}
