#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut vv = vec![0; m];
    for _ in 0..n {
        input! {
            k: usize,
            aa: [Usize1; k]
        }
        for a in aa {
            vv[a] += 1;
        }
    }
    println!("{}", vv.iter().filter(|&&v| v == n).count());
}
