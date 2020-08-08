#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut aabb: [(usize, usize); n]
    };
    aabb.sort_by_key(|ab| ab.0);
    let mut num_sum = 0;
    for (a, b) in aabb {
        if k <= num_sum + b {
            println!("{}", a);
            return;
        }
        num_sum += b;
    }
}
