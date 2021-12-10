#![allow(unused_imports)]
// use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut h: usize,
        mut aabb: [(usize, usize); n]
    };
    aabb.sort_unstable_by_key(|ab| ab.1);
    let max_a = aabb.iter().map(|ab| ab.0).max().unwrap();

    let mut rs = 0;
    for &(_a, b) in aabb.iter().rev() {
        if h == 0 || b <= max_a {
            break;
        }
        rs += 1;
        h = h.saturating_sub(b);
    }
    rs += h.div_ceil(&max_a);

    println!("{}", rs);
}
