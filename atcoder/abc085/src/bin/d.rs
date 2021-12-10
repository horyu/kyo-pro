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
    let mut max_a = 0;
    let mut max_a_index = 0;
    for (i, ab) in aabb.iter().enumerate() {
        if max_a < ab.0 {
            max_a = ab.0;
            max_a_index = i;
        }
    }
    let mut rs = 0;
    let max_a_min_b = aabb[max_a_index].1;
    for (i, &(_a, b)) in aabb.iter().enumerate().rev() {
        if max_a_index == i {
            continue;
        }
        if h <= max_a_min_b || b <= max_a {
            break;
        }
        rs += 1;
        h = h.saturating_sub(b);
    }
    if h != 0 {
        rs += 1 + h.saturating_sub(max_a_min_b).div_ceil(&max_a);
    }
    println!("{}", rs);
}
